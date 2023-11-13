export const idlFactory = ({ IDL }) => {
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const FlyInitData = IDL.Record({
    'minting_account' : IDL.Principal,
    'initial_balances' : IDL.Vec(IDL.Tuple(Account, IDL.Nat64)),
    'admins' : IDL.Vec(IDL.Principal),
    'total_supply' : IDL.Nat64,
  });
  const Role = IDL.Variant({ 'Admin' : IDL.Null });
  const ConfigurationError = IDL.Variant({
    'AdminsCantBeEmpty' : IDL.Null,
    'AnonymousAdmin' : IDL.Null,
  });
  const PoolError = IDL.Variant({
    'PoolNotFound' : IDL.Nat,
    'NotEnoughTokens' : IDL.Null,
  });
  const BalanceError = IDL.Variant({
    'AccountNotFound' : IDL.Null,
    'InsufficientBalance' : IDL.Null,
  });
  const FlyError = IDL.Variant({
    'Configuration' : ConfigurationError,
    'Pool' : PoolError,
    'StorageError' : IDL.Null,
    'Balance' : BalanceError,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : FlyError });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : FlyError });
  return IDL.Service({
    'admin_remove_role' : IDL.Func([IDL.Principal, Role], [Result], []),
    'admin_set_role' : IDL.Func([IDL.Principal, Role], [], []),
    'reserve_pool' : IDL.Func([IDL.Nat, IDL.Nat64], [Result_1], []),
  });
};
export const init = ({ IDL }) => {
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const FlyInitData = IDL.Record({
    'minting_account' : IDL.Principal,
    'initial_balances' : IDL.Vec(IDL.Tuple(Account, IDL.Nat64)),
    'admins' : IDL.Vec(IDL.Principal),
    'total_supply' : IDL.Nat64,
  });
  return [FlyInitData];
};
