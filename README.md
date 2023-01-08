# JunoBid smart contract

This contract allows for timed bidding session to be governed by a smart contract.

## Administration

On instantiation, a `Vec<Addr>` of admins is provided to the contract. These admins are able to start and end bidding session, as well as withdrawn funds from the contract once a bidding session is ended.

## Bidding periods

A bidding period is a data container for the following fields:

- Name and description of the bidding period (`name` & `description`, description optional)
- Expiry time, after which new bids cannot be submitted (`expires_at`)
- Minimum bid amount in units **^10\*6** (`minimum_bid`)
- Maximum amount of bids that can be accepted at the end of the bidding period (`accepted_bidders`)
- Denomination for the funds to be sent (`denom`)

To end a bidding period, an amount of winning bids over 1 and under the `accepted_bidders` config variable must be selected. Once a bidding period ends, funds from the accepted bids will be withdrawn to the withdrawal address specified in the message and all losing bidders will be refunded.

To start a bidding period, you'll need to provide some configuration options like so:

```json
{
  "start_bidding": {
    "bidding_period": { <BiddingPeriod> }
  }
}
```

You can end a bidding period anytime after you've created it by providing an array of bidders to accept and optionally an address to withdraw funds, like so:

```json
{
  "end_bidding": {
    "accepted_bids": [ "juno1abcdefg" ],
    "withdrawal_address": <optional>
  }
}
```

Funds will be withdrawn to the account executing the above message if a withdrawal address is not provided.

## Bidding

Bids can be submitted using the `Bid` message like so:

```json
{ "bid": {} }
```

To submit a bid on behalf of another address:

```json
{ "bid": { "address": "juno1abcdefg" } }
```

When sending a `Bid` message, funds corresponding to the `denom` of the bidding period must be sent along with the message. The minimum bid is also governed by the bidding period as `minimum_bid`.

## Querying the contract

The contract provides three queries:

- `bidding_period`, which requiures no arguments
- `bids`, which requires no arguments
- `bid`, which requires a bidder address

### BiddingPeriod

This query will return the current bidding period data, or `None` if there is no current bidding period.

### Bids

This query will return an array of all active bids.

### Bid

This query will return the data for a specific bid. You'll need to provide the address of the bidder whose data you'd like to query, like so:

```json
{ "bid": { "address": "juno1abcdefg" } }
```
