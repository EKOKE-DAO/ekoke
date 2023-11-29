import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export type BalanceError = { 'AccountNotFound' : null } |
  { 'InsufficientBalance' : null };
export type ConfigurationError = { 'AdminsCantBeEmpty' : null } |
  { 'AnonymousAdmin' : null };
export type FlyError = { 'Configuration' : ConfigurationError } |
  { 'Pool' : PoolError } |
  { 'Register' : RegisterError } |
  { 'StorageError' : null } |
  { 'Balance' : BalanceError };
export interface FlyInitData {
  'deferred_canister' : Principal,
  'initial_balances' : Array<[Account, bigint]>,
  'admins' : Array<Principal>,
  'total_supply' : bigint,
}
export type MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export type PoolError = { 'PoolNotFound' : bigint } |
  { 'NotEnoughTokens' : null };
export type RegisterError = { 'TransactionNotFound' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : FlyError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : FlyError };
export type Result_2 = { 'Ok' : Transaction } |
  { 'Err' : FlyError };
export type Result_3 = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export type Role = { 'DeferredCanister' : null } |
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
export interface _SERVICE {
  'admin_burn' : ActorMethod<[bigint], Result>,
  'admin_cycles' : ActorMethod<[], bigint>,
  'admin_remove_role' : ActorMethod<[Principal, Role], Result>,
  'admin_set_role' : ActorMethod<[Principal, Role], undefined>,
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
  'reserve_pool' : ActorMethod<[Account, bigint, bigint], Result_1>,
}
