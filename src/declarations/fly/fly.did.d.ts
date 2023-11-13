import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type ConfigurationError = { 'AdminsCantBeEmpty' : null } |
  { 'AnonymousAdmin' : null };
export type FlyError = { 'Configuration' : ConfigurationError } |
  { 'Pool' : PoolError } |
  { 'StorageError' : null };
export interface FlyInitData {
  'minting_account' : Principal,
  'admins' : Array<Principal>,
}
export type PoolError = { 'PoolNotFound' : bigint } |
  { 'NotEnoughTokens' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : FlyError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : FlyError };
export type Role = { 'Admin' : null };
export interface _SERVICE {
  'admin_remove_role' : ActorMethod<[Principal, Role], Result>,
  'admin_set_role' : ActorMethod<[Principal, Role], undefined>,
  'reserve_pool' : ActorMethod<[bigint, bigint], Result_1>,
}
