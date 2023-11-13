export const idlFactory = ({ IDL }) => {
  const FlyInitData = IDL.Record({
    'minting_account' : IDL.Principal,
    'admins' : IDL.Vec(IDL.Principal),
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
  const FlyError = IDL.Variant({
    'Configuration' : ConfigurationError,
    'Pool' : PoolError,
    'StorageError' : IDL.Null,
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
  const FlyInitData = IDL.Record({
    'minting_account' : IDL.Principal,
    'admins' : IDL.Vec(IDL.Principal),
  });
  return [FlyInitData];
};
