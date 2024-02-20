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
export interface Burn {
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
  'spender' : [] | [Account],
}
export interface EkokeIndexInitData { 'ledger_id' : Principal }
export interface GetAccountTransactionArgs {
  'max_results' : bigint,
  'start' : [] | [bigint],
  'account' : Account,
}
export interface GetTransactions {
  'transactions' : Array<TransactionWithId>,
  'oldest_tx_id' : [] | [bigint],
}
export interface GetTransactionsErr { 'message' : string }
export interface ListSubaccountsArgs {
  'owner' : Principal,
  'start' : [] | [Uint8Array | number[]],
}
export interface Mint {
  'to' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
  'amount' : bigint,
}
export type Result = { 'Ok' : GetTransactions } |
  { 'Err' : GetTransactionsErr };
export interface Transaction {
  'burn' : [] | [Burn],
  'kind' : string,
  'mint' : [] | [Mint],
  'approve' : [] | [Approve],
  'timestamp' : bigint,
  'transfer' : [] | [Transfer],
}
export interface TransactionWithId {
  'id' : bigint,
  'transaction' : Transaction,
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
export interface _SERVICE {
  'commit' : ActorMethod<[Transaction], bigint>,
  'get_account_transactions' : ActorMethod<[GetAccountTransactionArgs], Result>,
  'ledger_id' : ActorMethod<[], Principal>,
  'list_subaccounts' : ActorMethod<
    [ListSubaccountsArgs],
    Array<Uint8Array | number[]>
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
