export const idlFactory = ({ IDL }) => {
  const SellContractInitData = IDL.Record({
    'fly_canister' : IDL.Principal,
    'custodians' : IDL.Vec(IDL.Principal),
    'marketplace_canister' : IDL.Principal,
  });
  const BuildingData = IDL.Record({ 'city' : IDL.Text });
  const FlyError = IDL.Variant({ 'StorageError' : IDL.Null });
  const ConfigurationError = IDL.Variant({
    'CustodialsCantBeEmpty' : IDL.Null,
    'AnonymousCustodial' : IDL.Null,
  });
  const TokenError = IDL.Variant({
    'ContractValueIsNotMultipleOfInstallments' : IDL.Null,
    'TokenAlreadyExists' : IDL.Nat,
    'TokensMismatch' : IDL.Null,
    'ContractAlreadyExists' : IDL.Nat,
    'TokenDoesNotBelongToContract' : IDL.Nat,
    'TokenNotFound' : IDL.Nat,
    'ContractHasNoTokens' : IDL.Null,
    'TokenIsBurned' : IDL.Nat,
    'InvalidExpirationDate' : IDL.Null,
    'BadMintTokenOwner' : IDL.Nat,
  });
  const SellContractError = IDL.Variant({
    'Fly' : FlyError,
    'Configuration' : ConfigurationError,
    'Unauthorized' : IDL.Null,
    'Token' : TokenError,
    'StorageError' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : SellContractError });
  const Contract = IDL.Record({
    'id' : IDL.Nat,
    'value' : IDL.Nat64,
    'building' : BuildingData,
    'seller' : IDL.Principal,
    'expiration' : IDL.Text,
    'tokens' : IDL.Vec(IDL.Nat),
    'buyers' : IDL.Vec(IDL.Principal),
    'mfly_reward' : IDL.Nat64,
  });
  return IDL.Service({
    'admin_register_contract' : IDL.Func(
        [
          IDL.Nat,
          IDL.Principal,
          IDL.Vec(IDL.Principal),
          IDL.Text,
          IDL.Nat64,
          IDL.Nat64,
          BuildingData,
        ],
        [Result],
        [],
      ),
    'admin_set_fly_canister' : IDL.Func([IDL.Principal], [], []),
    'admin_set_marketplace_canister' : IDL.Func([IDL.Principal], [], []),
    'get_contract' : IDL.Func([IDL.Nat], [IDL.Opt(Contract)], ['query']),
    'get_contracts' : IDL.Func([], [IDL.Vec(IDL.Nat)], ['query']),
  });
};
export const init = ({ IDL }) => {
  const SellContractInitData = IDL.Record({
    'fly_canister' : IDL.Principal,
    'custodians' : IDL.Vec(IDL.Principal),
    'marketplace_canister' : IDL.Principal,
  });
  return [SellContractInitData];
};
