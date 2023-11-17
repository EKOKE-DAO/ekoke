export const idlFactory = ({ IDL }) => {
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const FlyInitData = IDL.Record({
    'deferred_canister' : IDL.Principal,
    'initial_balances' : IDL.Vec(IDL.Tuple(Account, IDL.Nat)),
    'admins' : IDL.Vec(IDL.Principal),
    'total_supply' : IDL.Nat64,
  });
  const ConfigurationError = IDL.Variant({
    'AdminsCantBeEmpty' : IDL.Null,
    'AnonymousAdmin' : IDL.Null,
  });
  const PoolError = IDL.Variant({
    'PoolNotFound' : IDL.Nat,
    'NotEnoughTokens' : IDL.Null,
  });
  const RegisterError = IDL.Variant({ 'TransactionNotFound' : IDL.Null });
  const BalanceError = IDL.Variant({
    'AccountNotFound' : IDL.Null,
    'InsufficientBalance' : IDL.Null,
  });
  const FlyError = IDL.Variant({
    'Configuration' : ConfigurationError,
    'Pool' : PoolError,
    'Register' : RegisterError,
    'StorageError' : IDL.Null,
    'Balance' : BalanceError,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : FlyError });
  const Role = IDL.Variant({
    'DeferredCanister' : IDL.Null,
    'Admin' : IDL.Null,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : FlyError });
  const Transaction = IDL.Record({
    'to' : Account,
    'fee' : IDL.Nat,
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at' : IDL.Nat64,
    'amount' : IDL.Nat,
  });
  const Result_2 = IDL.Variant({ 'Ok' : Transaction, 'Err' : FlyError });
  return IDL.Service({
    'admin_burn' : IDL.Func([IDL.Nat], [Result], []),
    'admin_cycles' : IDL.Func([], [IDL.Nat], ['query']),
    'admin_remove_role' : IDL.Func([IDL.Principal, Role], [Result], []),
    'admin_set_role' : IDL.Func([IDL.Principal, Role], [], []),
    'get_contract_reward' : IDL.Func([IDL.Nat, IDL.Nat], [Result_1], []),
    'get_transaction' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'reserve_pool' : IDL.Func([Account, IDL.Nat, IDL.Nat], [Result_1], []),
  });
};
export const init = ({ IDL }) => {
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const FlyInitData = IDL.Record({
    'deferred_canister' : IDL.Principal,
    'initial_balances' : IDL.Vec(IDL.Tuple(Account, IDL.Nat)),
    'admins' : IDL.Vec(IDL.Principal),
    'total_supply' : IDL.Nat64,
  });
  return [FlyInitData];
};
