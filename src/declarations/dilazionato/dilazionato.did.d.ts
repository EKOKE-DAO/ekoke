import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type ConfigurationError = { 'CustodialsCantBeEmpty' : null } |
  { 'AnonymousCustodial' : null };
export interface Contract {
  'id' : bigint,
  'value' : bigint,
  'type' : ContractType,
  'is_signed' : boolean,
  'properties' : Array<[string, GenericValue]>,
  'seller' : Principal,
  'expiration' : string,
  'tokens' : Array<bigint>,
  'currency' : string,
  'initial_value' : bigint,
  'buyers' : Array<Principal>,
}
export interface ContractRegistration {
  'id' : bigint,
  'value' : bigint,
  'type' : ContractType,
  'properties' : Array<[string, GenericValue]>,
  'seller' : Principal,
  'expiration' : string,
  'currency' : string,
  'installments' : bigint,
  'buyers' : Array<Principal>,
}
export type ContractType = { 'Sell' : null } |
  { 'Financing' : null };
export type DilazionatoError = { 'Fly' : FlyError } |
  { 'Configuration' : ConfigurationError } |
  { 'Unauthorized' : null } |
  { 'Token' : TokenError } |
  { 'StorageError' : null };
export interface DilazionatoInitData {
  'fly_canister' : Principal,
  'custodians' : Array<Principal>,
  'marketplace_canister' : Principal,
}
export type FlyError = { 'StorageError' : null };
export type GenericValue = { 'Nat64Content' : bigint } |
  { 'Nat32Content' : number } |
  { 'BoolContent' : boolean } |
  { 'Nat8Content' : number } |
  { 'Int64Content' : bigint } |
  { 'IntContent' : bigint } |
  { 'NatContent' : bigint } |
  { 'Nat16Content' : number } |
  { 'Int32Content' : number } |
  { 'Int8Content' : number } |
  { 'FloatContent' : number } |
  { 'Int16Content' : number } |
  { 'BlobContent' : Uint8Array | number[] } |
  { 'NestedContent' : Vec } |
  { 'Principal' : Principal } |
  { 'TextContent' : string };
export interface Metadata {
  'logo' : [] | [string],
  'name' : [] | [string],
  'created_at' : bigint,
  'upgraded_at' : bigint,
  'custodians' : Array<Principal>,
  'symbol' : [] | [string],
}
export type NftError = { 'UnauthorizedOperator' : null } |
  { 'SelfTransfer' : null } |
  { 'TokenNotFound' : null } |
  { 'UnauthorizedOwner' : null } |
  { 'TxNotFound' : null } |
  { 'SelfApprove' : null } |
  { 'OperatorNotFound' : null } |
  { 'ExistedNFT' : null } |
  { 'OwnerNotFound' : null } |
  { 'Other' : string };
export type Result = { 'Ok' : null } |
  { 'Err' : DilazionatoError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : NftError };
export type Result_2 = { 'Ok' : boolean } |
  { 'Err' : NftError };
export type Result_3 = { 'Ok' : [] | [Principal] } |
  { 'Err' : NftError };
export type Result_4 = { 'Ok' : Array<bigint> } |
  { 'Err' : NftError };
export type Result_5 = { 'Ok' : Array<TokenMetadata> } |
  { 'Err' : NftError };
export type Result_6 = { 'Ok' : TokenMetadata } |
  { 'Err' : NftError };
export type Result_7 = { 'Ok' : TxEvent } |
  { 'Err' : NftError };
export type Role = { 'Custodian' : null } |
  { 'Agent' : null };
export interface Stats {
  'cycles' : bigint,
  'total_transactions' : bigint,
  'total_unique_holders' : bigint,
  'total_supply' : bigint,
}
export type SupportedInterface = { 'Burn' : null } |
  { 'Mint' : null } |
  { 'Approval' : null } |
  { 'TransactionHistory' : null };
export type TokenError = { 'ContractAlreadySigned' : bigint } |
  { 'ContractValueIsNotMultipleOfInstallments' : null } |
  { 'TokenAlreadyExists' : bigint } |
  { 'TokensMismatch' : null } |
  { 'ContractAlreadyExists' : bigint } |
  { 'TokenDoesNotBelongToContract' : bigint } |
  { 'TokenNotFound' : bigint } |
  { 'ContractNotFound' : bigint } |
  { 'ContractNotSigned' : bigint } |
  { 'BadMintTokenOperator' : bigint } |
  { 'ContractHasNoTokens' : null } |
  { 'TokenIsBurned' : bigint } |
  { 'InvalidExpirationDate' : null } |
  { 'BadMintTokenOwner' : bigint };
export interface TokenMetadata {
  'transferred_at' : [] | [bigint],
  'transferred_by' : [] | [Principal],
  'owner' : [] | [Principal],
  'operator' : [] | [Principal],
  'approved_at' : [] | [bigint],
  'approved_by' : [] | [Principal],
  'properties' : Array<[string, GenericValue]>,
  'is_burned' : boolean,
  'token_identifier' : bigint,
  'burned_at' : [] | [bigint],
  'burned_by' : [] | [Principal],
  'minted_at' : bigint,
  'minted_by' : Principal,
}
export interface TxEvent {
  'time' : bigint,
  'operation' : string,
  'details' : Array<[string, GenericValue]>,
  'caller' : Principal,
}
export type Vec = Array<
  [
    string,
    { 'Nat64Content' : bigint } |
      { 'Nat32Content' : number } |
      { 'BoolContent' : boolean } |
      { 'Nat8Content' : number } |
      { 'Int64Content' : bigint } |
      { 'IntContent' : bigint } |
      { 'NatContent' : bigint } |
      { 'Nat16Content' : number } |
      { 'Int32Content' : number } |
      { 'Int8Content' : number } |
      { 'FloatContent' : number } |
      { 'Int16Content' : number } |
      { 'BlobContent' : Uint8Array | number[] } |
      { 'NestedContent' : Vec } |
      { 'Principal' : Principal } |
      { 'TextContent' : string },
  ]
>;
export interface _SERVICE {
  'admin_remove_role' : ActorMethod<[Principal, Role], Result>,
  'admin_set_fly_canister' : ActorMethod<[Principal], undefined>,
  'admin_set_marketplace_canister' : ActorMethod<[Principal], undefined>,
  'admin_set_role' : ActorMethod<[Principal, Role], undefined>,
  'admin_sign_contract' : ActorMethod<[bigint], Result>,
  'approve' : ActorMethod<[Principal, bigint], Result_1>,
  'balance_of' : ActorMethod<[Principal], Result_1>,
  'burn' : ActorMethod<[bigint], Result_1>,
  'custodians' : ActorMethod<[], Array<Principal>>,
  'cycles' : ActorMethod<[], bigint>,
  'get_contract' : ActorMethod<[bigint], [] | [Contract]>,
  'get_contracts' : ActorMethod<[], Array<bigint>>,
  'is_approved_for_all' : ActorMethod<[Principal, Principal], Result_2>,
  'logo' : ActorMethod<[], [] | [string]>,
  'metadata' : ActorMethod<[], Metadata>,
  'mint' : ActorMethod<
    [Principal, bigint, Array<[string, GenericValue]>],
    Result_1
  >,
  'name' : ActorMethod<[], [] | [string]>,
  'operator_of' : ActorMethod<[bigint], Result_3>,
  'operator_token_identifiers' : ActorMethod<[Principal], Result_4>,
  'operator_token_metadata' : ActorMethod<[Principal], Result_5>,
  'owner_of' : ActorMethod<[bigint], Result_3>,
  'owner_token_identifiers' : ActorMethod<[Principal], Result_4>,
  'owner_token_metadata' : ActorMethod<[Principal], Result_5>,
  'register_contract' : ActorMethod<[ContractRegistration], Result>,
  'seller_increment_contract_value' : ActorMethod<
    [bigint, bigint, bigint],
    Result
  >,
  'set_approval_for_all' : ActorMethod<[Principal, boolean], Result_1>,
  'set_custodians' : ActorMethod<[Array<Principal>], undefined>,
  'set_logo' : ActorMethod<[string], undefined>,
  'set_name' : ActorMethod<[string], undefined>,
  'set_symbol' : ActorMethod<[string], undefined>,
  'stats' : ActorMethod<[], Stats>,
  'supported_interfaces' : ActorMethod<[], Array<SupportedInterface>>,
  'symbol' : ActorMethod<[], [] | [string]>,
  'token_metadata' : ActorMethod<[bigint], Result_6>,
  'total_supply' : ActorMethod<[], bigint>,
  'total_transactions' : ActorMethod<[], bigint>,
  'total_unique_holders' : ActorMethod<[], bigint>,
  'transaction' : ActorMethod<[bigint], Result_7>,
  'transfer' : ActorMethod<[Principal, bigint], Result_1>,
  'transfer_from' : ActorMethod<[Principal, Principal, bigint], Result_1>,
  'update_contract_buyers' : ActorMethod<[bigint, Array<Principal>], Result>,
}
