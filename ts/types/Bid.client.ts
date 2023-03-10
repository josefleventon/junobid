/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { Addr, InstantiateMsg, ExecuteMsg, Timestamp, Uint64, Uint128, BiddingPeriod, QueryMsg, BidResponse, Bid, BiddingPeriodResponse, BidsResponse } from "./Bid.types";
export interface BidReadOnlyInterface {
  contractAddress: string;
  biddingPeriod: () => Promise<BiddingPeriodResponse>;
  bids: () => Promise<BidsResponse>;
  bid: ({
    address
  }: {
    address: Addr;
  }) => Promise<BidResponse>;
}
export class BidQueryClient implements BidReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.biddingPeriod = this.biddingPeriod.bind(this);
    this.bids = this.bids.bind(this);
    this.bid = this.bid.bind(this);
  }

  biddingPeriod = async (): Promise<BiddingPeriodResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      bidding_period: {}
    });
  };
  bids = async (): Promise<BidsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      bids: {}
    });
  };
  bid = async ({
    address
  }: {
    address: Addr;
  }): Promise<BidResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      bid: {
        address
      }
    });
  };
}
export interface BidInterface extends BidReadOnlyInterface {
  contractAddress: string;
  sender: string;
  startBidding: ({
    config
  }: {
    config: BiddingPeriod;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  endBidding: ({
    acceptedBids,
    withdrawalAddress
  }: {
    acceptedBids: Addr[];
    withdrawalAddress?: Addr;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  bid: ({
    address
  }: {
    address?: Addr;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class BidClient extends BidQueryClient implements BidInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.startBidding = this.startBidding.bind(this);
    this.endBidding = this.endBidding.bind(this);
    this.bid = this.bid.bind(this);
  }

  startBidding = async ({
    config
  }: {
    config: BiddingPeriod;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      start_bidding: {
        config
      }
    }, fee, memo, funds);
  };
  endBidding = async ({
    acceptedBids,
    withdrawalAddress
  }: {
    acceptedBids: Addr[];
    withdrawalAddress?: Addr;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      end_bidding: {
        accepted_bids: acceptedBids,
        withdrawal_address: withdrawalAddress
      }
    }, fee, memo, funds);
  };
  bid = async ({
    address
  }: {
    address?: Addr;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      bid: {
        address
      }
    }, fee, memo, funds);
  };
}