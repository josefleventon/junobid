use crate::state::{Bid, BiddingPeriod};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// - ADMIN FACING -
    /// Start a new bidding period
    StartBidding { config: BiddingPeriod },
    /// End the current bidding period
    /// This will return all the bids to the bidders, except the accepted bids
    /// All remaining balance will be withdrawn to the address that executed the msg,
    /// unless `withdrawal_address` is specified, in which case the funds will be
    /// transferred to that address
    EndBidding {
        accepted_bids: Vec<Addr>,
        withdrawal_address: Option<Addr>,
    },

    /// - BIDDER FACING -
    /// Create a new bid, optionally on behalf of another address
    Bid { address: Option<Addr> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BiddingPeriodResponse)]
    BiddingPeriod {},
    #[returns(BidsResponse)]
    Bids {},
    #[returns(BidResponse)]
    Bid { address: Addr },
}

// We define a custom struct for each query response

#[cw_serde]
pub struct BiddingPeriodResponse {
    pub bidding_period: Option<BiddingPeriod>,
}

#[cw_serde]
pub struct BidsResponse {
    pub bids: Vec<Bid>,
}

#[cw_serde]
pub struct BidResponse {
    pub bid: Option<Bid>,
}
