export const idlFactory = ({ IDL }) => {
  const Box = IDL.Rec();
  const Vec = IDL.Rec();
  const EkokeArchiveInitData = IDL.Record({
    'ledger_id' : IDL.Principal,
    'index_id' : IDL.Principal,
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
  const GetBlocksArg = IDL.Record({ 'start' : IDL.Nat, 'length' : IDL.Nat });
  Vec.fill(
    IDL.Vec(
      IDL.Variant({
        'Int' : IDL.Int,
        'Map' : IDL.Vec(IDL.Tuple(IDL.Text, Box)),
        'Nat' : IDL.Nat,
        'Nat64' : IDL.Nat64,
        'Blob' : IDL.Vec(IDL.Nat8),
        'Text' : IDL.Text,
        'Array' : Vec,
      })
    )
  );
  Box.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(IDL.Text, Box)),
      'Nat' : IDL.Nat,
      'Nat64' : IDL.Nat64,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Text' : IDL.Text,
      'Array' : Vec,
    })
  );
  const Value = IDL.Variant({
    'Int' : IDL.Int,
    'Map' : IDL.Vec(IDL.Tuple(IDL.Text, Box)),
    'Nat' : IDL.Nat,
    'Nat64' : IDL.Nat64,
    'Blob' : IDL.Vec(IDL.Nat8),
    'Text' : IDL.Text,
    'Array' : Vec,
  });
  const GetBlocksRet = IDL.Record({ 'blocks' : IDL.Vec(Value) });
  const GetTransactionsRet = IDL.Record({
    'transactions' : IDL.Vec(Transaction),
  });
  return IDL.Service({
    'append_blocks' : IDL.Func([IDL.Vec(IDL.Vec(IDL.Nat8))], [], []),
    'commit' : IDL.Func([Transaction], [IDL.Nat64], []),
    'get_blocks' : IDL.Func([GetBlocksArg], [GetBlocksRet], ['query']),
    'get_transaction' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(Transaction)],
        ['query'],
      ),
    'get_transactions' : IDL.Func(
        [GetBlocksArg],
        [GetTransactionsRet],
        ['query'],
      ),
    'remaining_capacity' : IDL.Func([], [IDL.Nat64], ['query']),
  });
};
export const init = ({ IDL }) => {
  const EkokeArchiveInitData = IDL.Record({
    'ledger_id' : IDL.Principal,
    'index_id' : IDL.Principal,
  });
  return [EkokeArchiveInitData];
};
