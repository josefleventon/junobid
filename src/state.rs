use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Bid {
    /// Address of the bidder
    pub bidder: Addr,
    /// Amount of tokens bid in *10^6 format
    pub amount: Uint128,
}

#[cw_serde]
pub struct BiddingPeriod {
    /// Name of the bidding period
    pub name: String,
    /// Optional description string
    pub description: Option<String>,
    /// Expiry time for the bidding period
    /// Bids made beyond this timestamp will not be accepted
    pub expires_at: Timestamp,
    /// Minimum bid amount
    pub minimum_bid: Uint128,
    /// Amount of bids that can be accepted once the bidding period is ended
    pub accepted_bidders: u64,
    /// Denomination in which bids are to be made
    pub denom: String,
}

pub const BIDS: Map<&Addr, Bid> = Map::new("bids");
pub const BIDDING_PERIOD: Item<BiddingPeriod> = Item::new("bidding_period");
