export const idlFactory = ({ IDL }) => {
  const Vec = IDL.Rec();
  const DeferredInitData = IDL.Record({
    'fly_canister' : IDL.Principal,
    'custodians' : IDL.Vec(IDL.Principal),
    'marketplace_canister' : IDL.Principal,
  });
  const Role = IDL.Variant({ 'Custodian' : IDL.Null, 'Agent' : IDL.Null });
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
  const ConfigurationError_1 = IDL.Variant({
    'CustodialsCantBeEmpty' : IDL.Null,
    'AnonymousCustodial' : IDL.Null,
  });
  const TokenError = IDL.Variant({
    'ContractAlreadySigned' : IDL.Nat,
    'ContractValueIsNotMultipleOfInstallments' : IDL.Null,
    'TokenAlreadyExists' : IDL.Nat,
    'TokensMismatch' : IDL.Null,
    'ContractAlreadyExists' : IDL.Nat,
    'ContractTokensShouldBeEmpty' : IDL.Null,
    'TokenDoesNotBelongToContract' : IDL.Nat,
    'TokenNotFound' : IDL.Nat,
    'ContractNotFound' : IDL.Nat,
    'ContractNotSigned' : IDL.Nat,
    'ContractHasNoTokens' : IDL.Null,
    'TokenIsBurned' : IDL.Nat,
    'InvalidExpirationDate' : IDL.Null,
    'BadMintTokenOwner' : IDL.Nat,
  });
  const DeferredError = IDL.Variant({
    'Fly' : FlyError,
    'Configuration' : ConfigurationError_1,
    'Unauthorized' : IDL.Null,
    'Token' : TokenError,
    'StorageError' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : DeferredError });
  const NftError = IDL.Variant({
    'UnauthorizedOperator' : IDL.Null,
    'SelfTransfer' : IDL.Null,
    'TokenNotFound' : IDL.Null,
    'UnauthorizedOwner' : IDL.Null,
    'TxNotFound' : IDL.Null,
    'SelfApprove' : IDL.Null,
    'OperatorNotFound' : IDL.Null,
    'ExistedNFT' : IDL.Null,
    'OwnerNotFound' : IDL.Null,
    'Other' : IDL.Text,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : NftError });
  const ContractType = IDL.Variant({
    'Sell' : IDL.Null,
    'Financing' : IDL.Null,
  });
  Vec.fill(
    IDL.Vec(
      IDL.Tuple(
        IDL.Text,
        IDL.Variant({
          'Nat64Content' : IDL.Nat64,
          'Nat32Content' : IDL.Nat32,
          'BoolContent' : IDL.Bool,
          'Nat8Content' : IDL.Nat8,
          'Int64Content' : IDL.Int64,
          'IntContent' : IDL.Int,
          'NatContent' : IDL.Nat,
          'Nat16Content' : IDL.Nat16,
          'Int32Content' : IDL.Int32,
          'Int8Content' : IDL.Int8,
          'FloatContent' : IDL.Float64,
          'Int16Content' : IDL.Int16,
          'BlobContent' : IDL.Vec(IDL.Nat8),
          'NestedContent' : Vec,
          'Principal' : IDL.Principal,
          'TextContent' : IDL.Text,
        }),
      )
    )
  );
  const GenericValue = IDL.Variant({
    'Nat64Content' : IDL.Nat64,
    'Nat32Content' : IDL.Nat32,
    'BoolContent' : IDL.Bool,
    'Nat8Content' : IDL.Nat8,
    'Int64Content' : IDL.Int64,
    'IntContent' : IDL.Int,
    'NatContent' : IDL.Nat,
    'Nat16Content' : IDL.Nat16,
    'Int32Content' : IDL.Int32,
    'Int8Content' : IDL.Int8,
    'FloatContent' : IDL.Float64,
    'Int16Content' : IDL.Int16,
    'BlobContent' : IDL.Vec(IDL.Nat8),
    'NestedContent' : Vec,
    'Principal' : IDL.Principal,
    'TextContent' : IDL.Text,
  });
  const Contract = IDL.Record({
    'id' : IDL.Nat,
    'value' : IDL.Nat64,
    'type' : ContractType,
    'is_signed' : IDL.Bool,
    'properties' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'seller' : IDL.Principal,
    'expiration' : IDL.Text,
    'tokens' : IDL.Vec(IDL.Nat),
    'currency' : IDL.Text,
    'installments' : IDL.Nat64,
    'initial_value' : IDL.Nat64,
    'buyers' : IDL.Vec(IDL.Principal),
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : NftError });
  const Metadata = IDL.Record({
    'logo' : IDL.Opt(IDL.Text),
    'name' : IDL.Opt(IDL.Text),
    'created_at' : IDL.Nat64,
    'upgraded_at' : IDL.Nat64,
    'custodians' : IDL.Vec(IDL.Principal),
    'symbol' : IDL.Opt(IDL.Text),
  });
  const Result_3 = IDL.Variant({
    'Ok' : IDL.Opt(IDL.Principal),
    'Err' : NftError,
  });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat), 'Err' : NftError });
  const TokenMetadata = IDL.Record({
    'transferred_at' : IDL.Opt(IDL.Nat64),
    'transferred_by' : IDL.Opt(IDL.Principal),
    'owner' : IDL.Opt(IDL.Principal),
    'operator' : IDL.Opt(IDL.Principal),
    'approved_at' : IDL.Opt(IDL.Nat64),
    'approved_by' : IDL.Opt(IDL.Principal),
    'properties' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'is_burned' : IDL.Bool,
    'token_identifier' : IDL.Nat,
    'burned_at' : IDL.Opt(IDL.Nat64),
    'burned_by' : IDL.Opt(IDL.Principal),
    'minted_at' : IDL.Nat64,
    'minted_by' : IDL.Principal,
  });
  const Result_5 = IDL.Variant({
    'Ok' : IDL.Vec(TokenMetadata),
    'Err' : NftError,
  });
  const ContractRegistration = IDL.Record({
    'id' : IDL.Nat,
    'value' : IDL.Nat64,
    'type' : ContractType,
    'properties' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'seller' : IDL.Principal,
    'expiration' : IDL.Text,
    'currency' : IDL.Text,
    'installments' : IDL.Nat64,
    'buyers' : IDL.Vec(IDL.Principal),
  });
  const Result_6 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : DeferredError });
  const Stats = IDL.Record({
    'cycles' : IDL.Nat,
    'total_transactions' : IDL.Nat,
    'total_unique_holders' : IDL.Nat,
    'total_supply' : IDL.Nat,
  });
  const SupportedInterface = IDL.Variant({
    'Burn' : IDL.Null,
    'Mint' : IDL.Null,
    'Approval' : IDL.Null,
    'TransactionHistory' : IDL.Null,
  });
  const Result_7 = IDL.Variant({ 'Ok' : TokenMetadata, 'Err' : NftError });
  const TxEvent = IDL.Record({
    'time' : IDL.Nat64,
    'operation' : IDL.Text,
    'details' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'caller' : IDL.Principal,
  });
  const Result_8 = IDL.Variant({ 'Ok' : TxEvent, 'Err' : NftError });
  return IDL.Service({
    'admin_get_unsigned_contracts' : IDL.Func(
        [],
        [IDL.Vec(IDL.Nat)],
        ['query'],
      ),
    'admin_remove_role' : IDL.Func([IDL.Principal, Role], [Result], []),
    'admin_set_fly_canister' : IDL.Func([IDL.Principal], [], []),
    'admin_set_marketplace_canister' : IDL.Func([IDL.Principal], [], []),
    'admin_set_role' : IDL.Func([IDL.Principal, Role], [], []),
    'admin_sign_contract' : IDL.Func([IDL.Nat], [Result], []),
    'approve' : IDL.Func([IDL.Principal, IDL.Nat], [Result_1], []),
    'balance_of' : IDL.Func([IDL.Principal], [Result_1], ['query']),
    'burn' : IDL.Func([IDL.Nat], [Result_1], []),
    'custodians' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'cycles' : IDL.Func([], [IDL.Nat], ['query']),
    'get_contract' : IDL.Func([IDL.Nat], [IDL.Opt(Contract)], ['query']),
    'get_signed_contracts' : IDL.Func([], [IDL.Vec(IDL.Nat)], ['query']),
    'is_approved_for_all' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [Result_2],
        [],
      ),
    'logo' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'metadata' : IDL.Func([], [Metadata], ['query']),
    'mint' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Vec(IDL.Tuple(IDL.Text, GenericValue))],
        [Result_1],
        [],
      ),
    'name' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'operator_of' : IDL.Func([IDL.Nat], [Result_3], ['query']),
    'operator_token_identifiers' : IDL.Func(
        [IDL.Principal],
        [Result_4],
        ['query'],
      ),
    'operator_token_metadata' : IDL.Func(
        [IDL.Principal],
        [Result_5],
        ['query'],
      ),
    'owner_of' : IDL.Func([IDL.Nat], [Result_3], ['query']),
    'owner_token_identifiers' : IDL.Func(
        [IDL.Principal],
        [Result_4],
        ['query'],
      ),
    'owner_token_metadata' : IDL.Func([IDL.Principal], [Result_5], ['query']),
    'register_contract' : IDL.Func([ContractRegistration], [Result_6], []),
    'seller_increment_contract_value' : IDL.Func(
        [IDL.Nat, IDL.Nat64, IDL.Nat64],
        [Result],
        [],
      ),
    'set_approval_for_all' : IDL.Func(
        [IDL.Principal, IDL.Bool],
        [Result_1],
        [],
      ),
    'set_custodians' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'set_logo' : IDL.Func([IDL.Text], [], []),
    'set_name' : IDL.Func([IDL.Text], [], []),
    'set_symbol' : IDL.Func([IDL.Text], [], []),
    'stats' : IDL.Func([], [Stats], ['query']),
    'supported_interfaces' : IDL.Func(
        [],
        [IDL.Vec(SupportedInterface)],
        ['query'],
      ),
    'symbol' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'token_metadata' : IDL.Func([IDL.Nat], [Result_7], ['query']),
    'total_supply' : IDL.Func([], [IDL.Nat], ['query']),
    'total_transactions' : IDL.Func([], [IDL.Nat], ['query']),
    'total_unique_holders' : IDL.Func([], [IDL.Nat], ['query']),
    'transaction' : IDL.Func([IDL.Nat], [Result_8], ['query']),
    'transfer' : IDL.Func([IDL.Principal, IDL.Nat], [Result_1], []),
    'transfer_from' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Nat],
        [Result_1],
        [],
      ),
    'update_contract_buyers' : IDL.Func(
        [IDL.Nat, IDL.Vec(IDL.Principal)],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const DeferredInitData = IDL.Record({
    'fly_canister' : IDL.Principal,
    'custodians' : IDL.Vec(IDL.Principal),
    'marketplace_canister' : IDL.Principal,
  });
  return [DeferredInitData];
};