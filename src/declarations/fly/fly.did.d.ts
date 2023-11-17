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
  'initial_balances' : Array<[Account, bigint]>,
  'dilazionato_canister' : Principal,
  'admins' : Array<Principal>,
  'total_supply' : bigint,
}
export type PoolError = { 'PoolNotFound' : bigint } |
  { 'NotEnoughTokens' : null };
export type RegisterError = { 'TransactionNotFound' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : FlyError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : FlyError };
export type Role = { 'Admin' : null } |
  { 'DilazionatoCanister' : null };
export interface _SERVICE {
  'admin_remove_role' : ActorMethod<[Principal, Role], Result>,
  'admin_set_role' : ActorMethod<[Principal, Role], undefined>,
  'get_contract_reward' : ActorMethod<[bigint, bigint], Result_1>,
  'reserve_pool' : ActorMethod<[Account, bigint, bigint], Result_1>,
}
