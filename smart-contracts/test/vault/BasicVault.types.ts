/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type ExecuteMsg = {
  bond: {
    recipient?: string | null;
  };
} | {
  unbond: {
    amount?: Uint128 | null;
  };
} | {
  claim: {};
} | {
  bond_response: BondResponse;
} | {
  start_unbond_response: StartUnbondResponse;
} | {
  unbond_response: UnbondResponse;
} | {
  transfer: {
    amount: Uint128;
    recipient: string;
  };
} | {
  burn: {
    amount: Uint128;
  };
} | {
  send: {
    amount: Uint128;
    contract: string;
    msg: Binary;
  };
} | {
  increase_allowance: {
    amount: Uint128;
    expires?: Expiration | null;
    spender: string;
  };
} | {
  decrease_allowance: {
    amount: Uint128;
    expires?: Expiration | null;
    spender: string;
  };
} | {
  transfer_from: {
    amount: Uint128;
    owner: string;
    recipient: string;
  };
} | {
  send_from: {
    amount: Uint128;
    contract: string;
    msg: Binary;
    owner: string;
  };
} | {
  burn_from: {
    amount: Uint128;
    owner: string;
  };
} | {
  clear_cache: {};
};
export type Uint128 = string;
export type Timestamp = Uint64;
export type Uint64 = string;
export type Binary = string;
export type Expiration = {
  at_height: number;
} | {
  at_time: Timestamp;
} | {
  never: {};
};
export interface BondResponse {
  bond_id: string;
  share_amount: Uint128;
  [k: string]: unknown;
}
export interface StartUnbondResponse {
  unbond_id: string;
  unlock_time: Timestamp;
  [k: string]: unknown;
}
export interface UnbondResponse {
  unbond_id: string;
  [k: string]: unknown;
}
export type PrimitiveInitMsg = {
  l_p: InstantiateMsg;
};
export type Decimal = string;
export type AssetInfoBaseForAddr = {
  native: string;
} | {
  cw20: Addr;
};
export type Addr = string;
export interface InstantiateMsg {
  base_denom: string;
  expected_connection: string;
  local_denom: string;
  lock_period: number;
  pool_denom: string;
  pool_id: number;
  quote_denom: string;
  return_source_channel: string;
  transfer_channel: string;
}
export interface PrimitiveConfig {
  address: string;
  init: PrimitiveInitMsg;
  weight: Decimal;
}
export interface InstantiateMsg1 {
  base_denom: string;
  expected_connection: string;
  local_denom: string;
  lock_period: number;
  pool_denom: string;
  pool_id: number;
  quote_denom: string;
  return_source_channel: string;
  transfer_channel: string;
}
export interface DistributionSchedule {
  amount: Uint128;
  end: number;
  start: number;
}
export type QueryMsg = {
  claims: {
    address: string;
  };
} | {
  investment: {};
} | {
  deposit_ratio: {
    funds: Coin[];
  };
} | {
  pending_bonds: {
    address: string;
  };
} | {
  get_tvl_info: {};
} | {
  pending_unbonds: {
    address: string;
  };
} | {
  get_debug: {};
} | {
  balance: {
    address: string;
  };
} | {
  token_info: {};
} | {
  additional_token_info: {};
} | {
  allowance: {
    owner: string;
    spender: string;
  };
};
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export interface VaultTokenInfoResponse {
  creation_time: Timestamp;
  decimals: number;
  name: string;
  symbol: string;
  thesis: string;
  total_supply: Uint128;
}
export interface AllowanceResponse {
  allowance: Uint128;
  expires: Expiration;
  [k: string]: unknown;
}
export interface BalanceResponse {
  balance: Uint128;
}
export interface ClaimsResponse {
  claims: Claim[];
}
export interface Claim {
  amount: Uint128;
  release_at: Expiration;
}
export interface DepositRatioResponse {
  primitive_funding_amounts: Coin[];
  remainder: Coin[];
}
export interface GetDebugResponse {
  debug: string;
}
export interface TvlInfoResponse {
  primitives: PrimitiveInfo[];
}
export interface PrimitiveInfo {
  base_denom: string;
  ica_address: string;
  lp_denom: string;
  lp_shares: LpCache;
  quote_denom: string;
}
export interface LpCache {
  d_unlocked_shares: Uint128;
  locked_shares: Uint128;
  w_unlocked_shares: Uint128;
  [k: string]: unknown;
}
export interface InvestmentResponse {
  info: InvestmentInfo;
}
export interface InvestmentInfo {
  min_withdrawal: Uint128;
  owner: Addr;
  primitives: PrimitiveConfig[];
}
export interface PendingBondsResponse {
  pending_bond_ids: string[];
  pending_bonds: BondingStub[];
}
export interface BondingStub {
  address: string;
  bond_response?: BondResponse | null;
}
export interface PendingUnbondsResponse {
  pending_unbond_ids: string[];
  pending_unbonds: Unbond[];
}
export interface Unbond {
  shares: Uint128;
  stub: UnbondingStub[];
}
export interface UnbondingStub {
  address: string;
  unbond_funds: Coin[];
  unbond_response?: UnbondResponse | null;
  unlock_time?: Timestamp | null;
}
export interface TokenInfoResponse {
  decimals: number;
  name: string;
  symbol: string;
  total_supply: Uint128;
}