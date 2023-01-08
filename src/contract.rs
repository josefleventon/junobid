#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_utils::must_pay;

use crate::admins::{can_execute, AdminList, ADMINS};
use crate::error::ContractError;
use crate::msg::{
    BidResponse, BiddingPeriodResponse, BidsResponse, ExecuteMsg, InstantiateMsg, QueryMsg,
};
use crate::state::{Bid, BiddingPeriod, BIDDING_PERIOD, BIDS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:juno_bid";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Save the list of admins
    ADMINS.save(deps.storage, &AdminList { admins: msg.admins })?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StartBidding { config } => execute_start_bidding(deps, env, info, config),
        ExecuteMsg::EndBidding {
            accepted_bids,
            withdrawal_address,
        } => execute_end_bidding(deps, env, info, accepted_bids, withdrawal_address),
        ExecuteMsg::Bid { address } => execute_bid(deps, env, info, address),
    }
}

fn execute_start_bidding(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    config: BiddingPeriod,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    };

    // Check if another bidding period already exists
    let bidding_period = BIDDING_PERIOD.may_load(deps.storage)?;
    if bidding_period.is_some() {
        return Err(ContractError::BiddingPeriodActive {});
    };

    // Verify that the date provided is not in the past
    if config.expires_at <= env.block.time {
        return Err(ContractError::CustomErrorParam {
            val: "Bidding period end time is in the past".into(),
        });
    };

    // Verify that at least 1 bid will be accepted
    if config.accepted_bidders < 1 {
        return Err(ContractError::CustomErrorParam {
            val: "At least 1 bid needs to be able to be accepted".into(),
        });
    }

    // If not, create the new bidding period
    BIDDING_PERIOD.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "start_bidding")
        .add_attribute("bidding_period_name", config.name)
        .add_attribute(
            "bidding_period_description",
            config.description.unwrap_or_else(|| "null".into()),
        )
        .add_attribute("bidding_period_expires_at", config.expires_at.to_string())
        .add_attribute(
            "bidding_period_accepted_bidders",
            config.accepted_bidders.to_string(),
        ))
}

fn execute_end_bidding(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    accepted_bids: Vec<Addr>,
    withdrawal_address: Option<Addr>,
) -> Result<Response, ContractError> {
    // Method is privileged
    if !can_execute(deps.as_ref(), info.sender.as_ref())? {
        return Err(ContractError::Unauthorized {});
    };

    // Verify that there is a bidding period to end
    let bidding_period = BIDDING_PERIOD.load(deps.storage)?;

    // Verify that the list of accepted bids is no longer than the config dictates
    if accepted_bids.len() as u64 > bidding_period.accepted_bidders {
        return Err(ContractError::CustomErrorParam {
            val: "You cannot accept more bids than the bidding period configuration allows".into(),
        });
    }

    // Remove current bidding period
    BIDDING_PERIOD.remove(deps.storage);

    // Keep the funds of all accepted bids
    let mut total_to_withdraw = Uint128::zero();
    for accepted_bid in accepted_bids {
        let bid = BIDS.load(deps.storage, &accepted_bid)?;
        total_to_withdraw += bid.amount;
        BIDS.remove(deps.storage, &accepted_bid);
    }

    // Reimburse all rejected bids
    let mut msgs: Vec<BankMsg> = vec![];
    let bid_keys = BIDS.keys(deps.storage, None, None, Order::Ascending);
    for key in bid_keys {
        match key {
            Ok(key) => {
                let bid = BIDS.load(deps.storage, &key)?;

                let refund_bidder = BankMsg::Send {
                    to_address: key.to_string(),
                    amount: vec![coin(bid.amount.u128(), bidding_period.denom.clone())],
                };
                msgs.push(refund_bidder);
            }
            Err(_) => return Err(ContractError::NotFound {}),
        }
    }

    // Clear the bids
    BIDS.clear(deps.storage);

    // Withdraw all remaining funds
    let withdrawal = BankMsg::Send {
        to_address: match withdrawal_address {
            Some(address) => address.into(),
            None => info.sender.into(),
        },
        amount: vec![coin(total_to_withdraw.u128(), bidding_period.denom)],
    };

    Ok(Response::new()
        .add_attribute("method", "end_bidding")
        .add_attribute("withdrawn", total_to_withdraw.to_string())
        .add_messages(msgs)
        .add_message(withdrawal))
}

fn execute_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: Option<Addr>,
) -> Result<Response, ContractError> {
    // There must be a current bidding period for a user to submit a bid
    let bidding_period = BIDDING_PERIOD.load(deps.storage)?;

    // Verify that the bidding period is not expired
    if env.block.time >= bidding_period.expires_at {
        return Err(ContractError::BiddingPeriodExpired {});
    }

    // Get the amount of tokens paid
    let amount_paid = must_pay(&info, &bidding_period.denom)?;

    // Verify that the amount paid is over the minimum bid amount
    if amount_paid < bidding_period.minimum_bid {
        return Err(ContractError::Payment(cw_utils::PaymentError::NoFunds {}));
    }

    let address = match address {
        Some(address) => address,
        None => info.sender,
    };

    // If the bid already exists, add on the amount sent
    // If not, create a new bid for the user
    let bid = BIDS.may_load(deps.storage, &address)?;
    match bid {
        Some(_) => BIDS.update(deps.storage, &address, |bid| match bid {
            Some(bid) => Ok(Bid {
                bidder: address.clone(),
                amount: bid.amount + amount_paid,
            }),
            None => Err(ContractError::NotFound {}),
        }),
        None => {
            let bid = Bid {
                bidder: address.clone(),
                amount: amount_paid,
            };
            let res = BIDS.save(deps.storage, &address, &bid);
            match res {
                Ok(_) => Ok(bid),
                Err(err) => Err(ContractError::Std(err)),
            }
        }
    }?;

    // Verify that the bid was created and get the "new amount"
    let bid = BIDS.load(deps.storage, &address)?;

    Ok(Response::new()
        .add_attribute("method", "bid")
        .add_attribute("address", address.to_string())
        .add_attribute("amount", amount_paid.to_string())
        .add_attribute("new_amount", bid.amount.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BiddingPeriod {} => to_binary(&query_bidding_period(deps)?),
        QueryMsg::Bids {} => to_binary(&query_bids(deps)?),
        QueryMsg::Bid { address } => to_binary(&query_bid(deps, address)?),
    }
}

fn query_bidding_period(deps: Deps) -> StdResult<BiddingPeriodResponse> {
    let bidding_period = BIDDING_PERIOD.may_load(deps.storage)?;
    Ok(BiddingPeriodResponse { bidding_period })
}

fn query_bids(deps: Deps) -> StdResult<BidsResponse> {
    let bids = BIDS
        .keys(deps.storage, None, None, Order::Descending)
        .map(|res| res.map(|addr| BIDS.load(deps.storage, &addr).unwrap()))
        .collect::<StdResult<Vec<_>>>()?;
    Ok(BidsResponse { bids })
}

fn query_bid(deps: Deps, address: Addr) -> StdResult<BidResponse> {
    let bid = BIDS.may_load(deps.storage, &address)?;
    Ok(BidResponse { bid })
}
