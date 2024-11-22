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
export type ConfigurationError = { 'AnonymousOwner' : null } |
  { 'AnonymousMinter' : null };
export type Continent = { 'Africa' : null } |
  { 'Antarctica' : null } |
  { 'Asia' : null } |
  { 'Europe' : null } |
  { 'SouthAmerica' : null } |
  { 'Oceania' : null } |
  { 'NorthAmerica' : null };
export interface Contract {
  'id' : bigint,
  'closed' : boolean,
  'value' : bigint,
  'type' : ContractType,
  'agency' : [] | [Agency],
  'restricted_properties' : Array<[string, RestrictedProperty]>,
  'properties' : Array<[string, GenericValue]>,
  'deposit' : bigint,
  'sellers' : Array<Seller>,
  'expiration' : string,
  'currency' : string,
  'installments' : bigint,
  'buyers' : Array<string>,
}
export type ContractError = { 'ContractIsClosed' : bigint } |
  { 'ContractNotFound' : bigint } |
  { 'BadContractProperty' : null };
export type ContractType = { 'Sell' : null } |
  { 'Financing' : null };
export type DeferredDataError = { 'Configuration' : ConfigurationError } |
  { 'Contract' : ContractError } |
  { 'InvalidSignature' : null } |
  { 'Unauthorized' : null } |
  { 'StorageError' : null } |
  { 'CanisterCall' : [RejectionCode, string] };
export interface DeferredDataInitData { 'minter' : Principal }
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
  { 'NestedContent' : Array<[string, GenericValue]> } |
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
  { 'Err' : DeferredDataError };
export interface Seller { 'quota' : number, 'address' : string }
export interface _SERVICE {
  'admin_cycles' : ActorMethod<[], bigint>,
  'admin_set_minter' : ActorMethod<[Principal], Result>,
  'get_contract' : ActorMethod<[bigint], [] | [Contract]>,
  'get_contracts' : ActorMethod<[], Array<bigint>>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'minter_close_contract' : ActorMethod<[bigint], Result>,
  'minter_create_contract' : ActorMethod<[Contract], Result>,
  'update_contract_property' : ActorMethod<
    [bigint, string, GenericValue],
    Result
  >,
  'update_restricted_contract_property' : ActorMethod<
    [bigint, string, RestrictedProperty],
    Result
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
