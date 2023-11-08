import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface BuildingData { 'city' : string }
export type ConfigurationError = { 'CustodialsCantBeEmpty' : null } |
  { 'AnonymousCustodial' : null };
export interface Contract {
  'id' : bigint,
  'value' : bigint,
  'building' : BuildingData,
  'seller' : Principal,
  'expiration' : string,
  'tokens' : Array<bigint>,
  'buyers' : Array<Principal>,
  'mfly_reward' : bigint,
}
export type FlyError = { 'StorageError' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : SellContractError };
export type SellContractError = { 'Fly' : FlyError } |
  { 'Configuration' : ConfigurationError } |
  { 'Unauthorized' : null } |
  { 'Token' : TokenError } |
  { 'StorageError' : null };
export interface SellContractInitData {
  'fly_canister' : Principal,
  'custodians' : Array<Principal>,
  'marketplace_canister' : Principal,
}
export type TokenError = { 'ContractValueIsNotMultipleOfInstallments' : null } |
  { 'TokenAlreadyExists' : bigint } |
  { 'TokensMismatch' : null } |
  { 'ContractAlreadyExists' : bigint } |
  { 'TokenDoesNotBelongToContract' : bigint } |
  { 'TokenNotFound' : bigint } |
  { 'ContractHasNoTokens' : null } |
  { 'TokenIsBurned' : bigint } |
  { 'InvalidExpirationDate' : null } |
  { 'BadMintTokenOwner' : bigint };
export interface _SERVICE {
  'admin_register_contract' : ActorMethod<
    [bigint, Principal, Array<Principal>, string, bigint, bigint, BuildingData],
    Result
  >,
  'admin_set_fly_canister' : ActorMethod<[Principal], undefined>,
  'admin_set_marketplace_canister' : ActorMethod<[Principal], undefined>,
  'get_contract' : ActorMethod<[bigint], [] | [Contract]>,
  'get_contracts' : ActorMethod<[], Array<bigint>>,
}
