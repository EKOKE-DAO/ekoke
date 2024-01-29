import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface Allowance {
  'allowance' : bigint,
  'expires_at' : [] | [bigint],
}
export interface AllowanceArgs { 'account' : Account, 'spender' : Account }
export type AllowanceError = { 'AllowanceNotFound' : null } |
  { 'BadSpender' : null } |
  { 'AllowanceChanged' : null } |
  { 'BadExpiration' : null } |
  { 'AllowanceExpired' : null } |
  { 'InsufficientFunds' : null };
export interface ApproveArgs {
  'fee' : [] | [bigint],
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'expected_allowance' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : Account,
}
export type ApproveError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'AllowanceChanged' : { 'current_allowance' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'Expired' : { 'ledger_time' : bigint } } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export type BalanceError = { 'AccountNotFound' : null } |
  { 'InsufficientBalance' : null };
export type ConfigurationError = { 'AdminsCantBeEmpty' : null } |
  { 'AnonymousAdmin' : null };
export type EkokeError = { 'Configuration' : ConfigurationError } |
  { 'Icrc1Transfer' : TransferError } |
  { 'Pool' : PoolError } |
  { 'Allowance' : AllowanceError } |
  { 'Register' : RegisterError } |
  { 'XrcError' : null } |
  { 'StorageError' : null } |
  { 'CanisterCall' : [RejectionCode, string] } |
  { 'Balance' : BalanceError } |
  { 'Icrc2Transfer' : TransferFromError };
export interface EkokeInitData {
  'deferred_canister' : Principal,
  'icp_ledger_canister' : Principal,
  'minting_account' : Account,
  'ckbtc_canister' : Principal,
  'initial_balances' : Array<[Account, bigint]>,
  'swap_account' : Account,
  'xrc_canister' : Principal,
  'marketplace_canister' : Principal,
  'admins' : Array<Principal>,
  'total_supply' : bigint,
}
export interface LiquidityPoolAccounts { 'icp' : Account, 'ckbtc' : Account }
export interface LiquidityPoolBalance { 'icp' : bigint, 'ckbtc' : bigint }
export type MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export type PoolError = { 'PoolNotFound' : bigint } |
  { 'NotEnoughTokens' : null };
export type RegisterError = { 'TransactionNotFound' : null };
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : EkokeError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : EkokeError };
export type Result_2 = { 'Ok' : Transaction } |
  { 'Err' : EkokeError };
export type Result_3 = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export type Result_4 = { 'Ok' : bigint } |
  { 'Err' : ApproveError };
export type Result_5 = { 'Ok' : bigint } |
  { 'Err' : TransferFromError };
export type Result_6 = { 'Ok' : LiquidityPoolBalance } |
  { 'Err' : EkokeError };
export type Role = { 'DeferredCanister' : null } |
  { 'MarketplaceCanister' : null } |
  { 'Admin' : null };
export interface TokenExtension { 'url' : string, 'name' : string }
export interface Transaction {
  'to' : Account,
  'fee' : bigint,
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at' : bigint,
  'amount' : bigint,
}
export interface TransferArg {
  'to' : Account,
  'fee' : [] | [bigint],
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export type TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface TransferFromArgs {
  'to' : Account,
  'fee' : [] | [bigint],
  'spender_subaccount' : [] | [Uint8Array | number[]],
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export type TransferFromError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'InsufficientAllowance' : { 'allowance' : bigint } } |
  { 'BadBurn' : { 'min_burn_amount' : bigint } } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'BadFee' : { 'expected_fee' : bigint } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : bigint } };
export interface _SERVICE {
  'admin_burn' : ActorMethod<[bigint], Result>,
  'admin_cycles' : ActorMethod<[], bigint>,
  'admin_remove_role' : ActorMethod<[Principal, Role], Result>,
  'admin_set_ckbtc_canister' : ActorMethod<[Principal], undefined>,
  'admin_set_icp_ledger_canister' : ActorMethod<[Principal], undefined>,
  'admin_set_role' : ActorMethod<[Principal, Role], undefined>,
  'admin_set_swap_account' : ActorMethod<[Account], undefined>,
  'admin_set_xrc_canister' : ActorMethod<[Principal], undefined>,
  'get_contract_reward' : ActorMethod<[bigint, bigint], Result_1>,
  'get_transaction' : ActorMethod<[bigint], Result_2>,
  'icrc1_balance_of' : ActorMethod<[Account], bigint>,
  'icrc1_decimals' : ActorMethod<[], number>,
  'icrc1_fee' : ActorMethod<[], bigint>,
  'icrc1_metadata' : ActorMethod<[], Array<[string, MetadataValue]>>,
  'icrc1_name' : ActorMethod<[], string>,
  'icrc1_supported_standards' : ActorMethod<[], Array<TokenExtension>>,
  'icrc1_symbol' : ActorMethod<[], string>,
  'icrc1_total_supply' : ActorMethod<[], bigint>,
  'icrc1_transfer' : ActorMethod<[TransferArg], Result_3>,
  'icrc2_allowance' : ActorMethod<[AllowanceArgs], Allowance>,
  'icrc2_approve' : ActorMethod<[ApproveArgs], Result_4>,
  'icrc2_transfer_from' : ActorMethod<[TransferFromArgs], Result_5>,
  'liquidity_pool_accounts' : ActorMethod<[], LiquidityPoolAccounts>,
  'liquidity_pool_balance' : ActorMethod<[], Result_6>,
  'reserve_pool' : ActorMethod<[Account, bigint, bigint], Result_1>,
  'send_reward' : ActorMethod<[bigint, bigint, Account], Result>,
}
