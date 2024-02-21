export const idlFactory = ({ IDL }) => {
  const EkokeIndexInitData = IDL.Record({
    'ledger_id' : IDL.Principal,
    'archive_id' : IDL.Principal,
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const Burn = IDL.Record({
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'spender' : IDL.Opt(Account),
  });
  const Mint = IDL.Record({
    'to' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
  });
  const Approve = IDL.Record({
    'fee' : IDL.Opt(IDL.Nat),
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'expected_allowance' : IDL.Opt(IDL.Nat),
    'expires_at' : IDL.Opt(IDL.Nat64),
    'spender' : IDL.Opt(Account),
  });
  const Transfer = IDL.Record({
    'to' : Account,
    'fee' : IDL.Opt(IDL.Nat),
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'spender' : IDL.Opt(Account),
  });
  const Transaction = IDL.Record({
    'burn' : IDL.Opt(Burn),
    'kind' : IDL.Text,
    'mint' : IDL.Opt(Mint),
    'approve' : IDL.Opt(Approve),
    'timestamp' : IDL.Nat64,
    'transfer' : IDL.Opt(Transfer),
  });
  const GetAccountTransactionArgs = IDL.Record({
    'max_results' : IDL.Nat,
    'start' : IDL.Opt(IDL.Nat),
    'account' : Account,
  });
  const TransactionWithId = IDL.Record({
    'id' : IDL.Nat,
    'transaction' : Transaction,
  });
  const GetTransactions = IDL.Record({
    'transactions' : IDL.Vec(TransactionWithId),
    'oldest_tx_id' : IDL.Opt(IDL.Nat),
  });
  const GetTransactionsErr = IDL.Record({ 'message' : IDL.Text });
  const Result = IDL.Variant({
    'Ok' : GetTransactions,
    'Err' : GetTransactionsErr,
  });
  const ListSubaccountsArgs = IDL.Record({
    'owner' : IDL.Principal,
    'start' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  return IDL.Service({
    'commit' : IDL.Func([IDL.Nat64, Transaction], [IDL.Nat], []),
    'get_account_transactions' : IDL.Func(
        [GetAccountTransactionArgs],
        [Result],
        [],
      ),
    'ledger_id' : IDL.Func([], [IDL.Principal], ['query']),
    'list_subaccounts' : IDL.Func(
        [ListSubaccountsArgs],
        [IDL.Vec(IDL.Vec(IDL.Nat8))],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => {
  const EkokeIndexInitData = IDL.Record({
    'ledger_id' : IDL.Principal,
    'archive_id' : IDL.Principal,
  });
  return [EkokeIndexInitData];
};
