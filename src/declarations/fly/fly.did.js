export const idlFactory = ({ IDL }) => {
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const FlyInitData = IDL.Record({
    'initial_balances' : IDL.Vec(IDL.Tuple(Account, IDL.Nat)),
    'dilazionato_canister' : IDL.Principal,
    'admins' : IDL.Vec(IDL.Principal),
    'total_supply' : IDL.Nat64,
  });
  const Role = IDL.Variant({
    'Admin' : IDL.Null,
    'DilazionatoCanister' : IDL.Null,
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
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : FlyError });
  return IDL.Service({
    'admin_remove_role' : IDL.Func([IDL.Principal, Role], [Result], []),
    'admin_set_role' : IDL.Func([IDL.Principal, Role], [], []),
    'get_contract_reward' : IDL.Func([IDL.Nat, IDL.Nat], [Result_1], []),
    'reserve_pool' : IDL.Func([Account, IDL.Nat, IDL.Nat], [Result_1], []),
  });
};
export const init = ({ IDL }) => {
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const FlyInitData = IDL.Record({
    'initial_balances' : IDL.Vec(IDL.Tuple(Account, IDL.Nat)),
    'dilazionato_canister' : IDL.Principal,
    'admins' : IDL.Vec(IDL.Principal),
    'total_supply' : IDL.Nat64,
  });
  return [FlyInitData];
};
