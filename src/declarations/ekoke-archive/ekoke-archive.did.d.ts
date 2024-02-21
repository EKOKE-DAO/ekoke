import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface Approve {
  'fee' : [] | [bigint],
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'expected_allowance' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : [] | [Account],
}
export type Box = { 'Int' : bigint } |
  { 'Map' : Array<[string, Box]> } |
  { 'Nat' : bigint } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Vec };
export interface Burn {
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'spender' : [] | [Account],
}
export interface EkokeArchiveInitData {
  'ledger_id' : Principal,
  'index_id' : Principal,
}
export interface GetBlocksArg { 'start' : bigint, 'length' : bigint }
export interface GetBlocksRet { 'blocks' : Array<Value> }
export interface GetTransactionsRet { 'transactions' : Array<Transaction> }
export interface Mint {
  'to' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export interface Transaction {
  'burn' : [] | [Burn],
  'kind' : string,
  'mint' : [] | [Mint],
  'approve' : [] | [Approve],
  'timestamp' : bigint,
  'transfer' : [] | [Transfer],
}
export interface Transfer {
  'to' : Account,
  'fee' : [] | [bigint],
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'spender' : [] | [Account],
}
export type Value = { 'Int' : bigint } |
  { 'Map' : Array<[string, Box]> } |
  { 'Nat' : bigint } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Vec };
export type Vec = Array<
  { 'Int' : bigint } |
    { 'Map' : Array<[string, Box]> } |
    { 'Nat' : bigint } |
    { 'Nat64' : bigint } |
    { 'Blob' : Uint8Array | number[] } |
    { 'Text' : string } |
    { 'Array' : Vec }
>;
export interface _SERVICE {
  'append_blocks' : ActorMethod<[Array<Uint8Array | number[]>], undefined>,
  'commit' : ActorMethod<[Transaction], bigint>,
  'get_blocks' : ActorMethod<[GetBlocksArg], GetBlocksRet>,
  'get_transaction' : ActorMethod<[bigint], [] | [Transaction]>,
  'get_transactions' : ActorMethod<[GetBlocksArg], GetTransactionsRet>,
  'remaining_capacity' : ActorMethod<[], bigint>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
