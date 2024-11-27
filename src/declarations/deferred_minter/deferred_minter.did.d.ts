import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Agency {
  'vat' : string,
  'region' : string,
  'zip_code' : string,
  'country' : string,
  'agent' : string,
  'owner' : Principal,
  'city' : string,
  'logo' : [] | [string],
  'name' : string,
  'continent' : Continent,
  'email' : string,
  'website' : string,
  'address' : string,
  'mobile' : string,
}
export type CloseContractError = { 'ContractNotFound' : bigint } |
  { 'ContractNotExpired' : bigint };
export type ConfigurationError = { 'CustodialsCantBeEmpty' : null } |
  { 'AnonymousCustodial' : null };
export type ConfigurationError_1 = { 'AnonymousOwner' : null } |
  { 'AnonymousMinter' : null };
export type Continent = { 'Africa' : null } |
  { 'Antarctica' : null } |
  { 'Asia' : null } |
  { 'Europe' : null } |
  { 'SouthAmerica' : null } |
  { 'Oceania' : null } |
  { 'NorthAmerica' : null };
export type ContractError = { 'CurrencyNotAllowed' : string } |
  { 'ContractValueIsNotMultipleOfInstallments' : null } |
  { 'ContractSellerQuotaIsNot100' : null } |
  { 'ContractPriceMismatch' : null } |
  { 'TokenValueIsZero' : null } |
  { 'ContractNotFound' : bigint } |
  { 'CannotCloseContract' : null } |
  { 'ContractHasNoSeller' : null } |
  { 'ContractHasNoBuyer' : null } |
  { 'BadContractExpiration' : null } |
  { 'ContractHasNoTokens' : null } |
  { 'BadContractProperty' : null };
export type ContractError_1 = { 'DocumentNotFound' : bigint } |
  { 'ContractNotFound' : bigint } |
  { 'BadContractProperty' : null };
export interface ContractRegistration {
  'value' : bigint,
  'type' : ContractType,
  'restricted_properties' : Array<[string, RestrictedProperty]>,
  'properties' : Array<[string, GenericValue]>,
  'deposit' : bigint,
  'sellers' : Array<Seller>,
  'token_value' : bigint,
  'expiration' : string,
  'currency' : string,
  'installments' : bigint,
  'buyers' : Array<string>,
}
export type ContractType = { 'Sell' : null } |
  { 'Financing' : null };
export type DeferredDataError = { 'Configuration' : ConfigurationError_1 } |
  { 'Contract' : ContractError_1 } |
  { 'InvalidSignature' : null } |
  { 'Unauthorized' : null } |
  { 'StorageError' : null } |
  { 'CanisterCall' : [RejectionCode, string] };
export type DeferredMinterError = { 'Configuration' : ConfigurationError } |
  { 'Contract' : ContractError } |
  { 'CloseContract' : CloseContractError } |
  { 'Unauthorized' : null } |
  { 'FailedToDecodeOutput' : string } |
  { 'EvmRpc' : string } |
  { 'DataCanister' : DeferredDataError } |
  { 'StorageError' : null } |
  { 'CanisterCall' : [RejectionCode, string] } |
  { 'Ecdsa' : EcdsaError };
export interface DeferredMinterInitData {
  'deferred_erc721' : string,
  'evm_rpc_api' : [] | [string],
  'allowed_currencies' : Array<string>,
  'deferred_data' : Principal,
  'reward_pool' : string,
  'custodians' : Array<Principal>,
  'chain_id' : bigint,
  'evm_rpc' : Principal,
  'ecdsa_key' : EcdsaKey,
  'log_settings' : LogSettingsV2,
}
export type EcdsaError = { 'RecoveryIdError' : string } |
  { 'InvalidSignature' : string } |
  { 'InvalidPublicKey' : string };
export type EcdsaKey = { 'Dfx' : null } |
  { 'Production' : null } |
  { 'Test' : null };
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
  { 'Principal' : Principal } |
  { 'TextContent' : string };
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
  'upgrade' : [] | [boolean],
  'status_code' : number,
}
export interface Log { 'log' : string, 'offset' : bigint }
export interface LogSettingsV2 {
  'log_filter' : string,
  'in_memory_records' : bigint,
  'enable_console' : boolean,
  'max_record_length' : bigint,
}
export interface Logs { 'logs' : Array<Log>, 'all_logs_count' : bigint }
export interface Pagination { 'count' : bigint, 'offset' : bigint }
export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export interface RestrictedProperty {
  'value' : GenericValue,
  'access_list' : Array<RestrictionLevel>,
}
export type RestrictionLevel = { 'Buyer' : null } |
  { 'Seller' : null } |
  { 'Agent' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : DeferredMinterError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : DeferredMinterError };
export type Result_2 = { 'Ok' : string } |
  { 'Err' : DeferredMinterError };
export type Role = { 'Custodian' : null } |
  { 'Agent' : null } |
  { 'GasStation' : null };
export interface Seller { 'quota' : number, 'address' : string }
export interface _SERVICE {
  'admin_cycles' : ActorMethod<[], bigint>,
  'admin_ic_logs' : ActorMethod<[Pagination], Logs>,
  'admin_register_agency' : ActorMethod<[Principal, Agency], undefined>,
  'admin_remove_role' : ActorMethod<[Principal, Role], Result>,
  'admin_set_allowed_currencies' : ActorMethod<[Array<string>], undefined>,
  'admin_set_custodians' : ActorMethod<[Array<Principal>], Result>,
  'admin_set_role' : ActorMethod<[Principal, Role], undefined>,
  'close_contract' : ActorMethod<[bigint], Result>,
  'create_contract' : ActorMethod<[ContractRegistration], Result_1>,
  'gas_station_set_gas_price' : ActorMethod<[bigint], Result>,
  'get_agencies' : ActorMethod<[], Array<Agency>>,
  'get_eth_address' : ActorMethod<[], Result_2>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'remove_agency' : ActorMethod<[Principal], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
