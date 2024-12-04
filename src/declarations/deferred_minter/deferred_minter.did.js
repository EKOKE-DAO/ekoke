export const idlFactory = ({ IDL }) => {
  const EcdsaKey = IDL.Variant({
    'Dfx' : IDL.Null,
    'Production' : IDL.Null,
    'Test' : IDL.Null,
  });
  const LogSettingsV2 = IDL.Record({
    'log_filter' : IDL.Text,
    'in_memory_records' : IDL.Nat64,
    'enable_console' : IDL.Bool,
    'max_record_length' : IDL.Nat64,
  });
  const DeferredMinterInitData = IDL.Record({
    'deferred_erc721' : IDL.Text,
    'evm_rpc_api' : IDL.Opt(IDL.Text),
    'allowed_currencies' : IDL.Vec(IDL.Text),
    'deferred_data' : IDL.Principal,
    'reward_pool' : IDL.Text,
    'custodians' : IDL.Vec(IDL.Principal),
    'chain_id' : IDL.Nat64,
    'evm_rpc' : IDL.Principal,
    'ecdsa_key' : EcdsaKey,
    'log_settings' : LogSettingsV2,
  });
  const Pagination = IDL.Record({ 'count' : IDL.Nat64, 'offset' : IDL.Nat64 });
  const Log = IDL.Record({ 'log' : IDL.Text, 'offset' : IDL.Nat64 });
  const Logs = IDL.Record({
    'logs' : IDL.Vec(Log),
    'all_logs_count' : IDL.Nat64,
  });
  const Continent = IDL.Variant({
    'Africa' : IDL.Null,
    'Antarctica' : IDL.Null,
    'Asia' : IDL.Null,
    'Europe' : IDL.Null,
    'SouthAmerica' : IDL.Null,
    'Oceania' : IDL.Null,
    'NorthAmerica' : IDL.Null,
  });
  const Agency = IDL.Record({
    'vat' : IDL.Text,
    'region' : IDL.Text,
    'zip_code' : IDL.Text,
    'country' : IDL.Text,
    'agent' : IDL.Text,
    'owner' : IDL.Principal,
    'city' : IDL.Text,
    'logo' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'continent' : Continent,
    'email' : IDL.Text,
    'website' : IDL.Text,
    'address' : IDL.Text,
    'mobile' : IDL.Text,
  });
  const Role = IDL.Variant({
    'Custodian' : IDL.Null,
    'Agent' : IDL.Null,
    'GasStation' : IDL.Null,
  });
  const ConfigurationError = IDL.Variant({
    'CustodialsCantBeEmpty' : IDL.Null,
    'AnonymousCustodial' : IDL.Null,
  });
  const ContractError = IDL.Variant({
    'CurrencyNotAllowed' : IDL.Text,
    'ContractValueIsNotMultipleOfInstallments' : IDL.Null,
    'ContractSellerQuotaIsNot100' : IDL.Null,
    'ContractPriceMismatch' : IDL.Null,
    'TokenValueIsZero' : IDL.Null,
    'ContractNotFound' : IDL.Nat,
    'CannotCloseContract' : IDL.Null,
    'ContractHasNoSeller' : IDL.Null,
    'ContractHasNoBuyer' : IDL.Null,
    'BadContractExpiration' : IDL.Null,
    'ContractHasNoTokens' : IDL.Null,
    'BadContractProperty' : IDL.Null,
  });
  const CloseContractError = IDL.Variant({
    'ContractNotFound' : IDL.Nat,
    'ContractNotExpired' : IDL.Nat,
  });
  const ConfigurationError_1 = IDL.Variant({
    'AnonymousOwner' : IDL.Null,
    'AnonymousMinter' : IDL.Null,
  });
  const ContractError_1 = IDL.Variant({
    'DocumentNotFound' : IDL.Nat,
    'ContractNotFound' : IDL.Nat,
    'BadContractProperty' : IDL.Null,
  });
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const DeferredDataError = IDL.Variant({
    'Configuration' : ConfigurationError_1,
    'Contract' : ContractError_1,
    'InvalidSignature' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'StorageError' : IDL.Null,
    'CanisterCall' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const EcdsaError = IDL.Variant({
    'RecoveryIdError' : IDL.Text,
    'InvalidSignature' : IDL.Text,
    'InvalidPublicKey' : IDL.Text,
  });
  const DeferredMinterError = IDL.Variant({
    'Configuration' : ConfigurationError,
    'Contract' : ContractError,
    'CloseContract' : CloseContractError,
    'Unauthorized' : IDL.Null,
    'FailedToDecodeOutput' : IDL.Text,
    'EvmRpc' : IDL.Text,
    'DataCanister' : DeferredDataError,
    'StorageError' : IDL.Null,
    'CanisterCall' : IDL.Tuple(RejectionCode, IDL.Text),
    'Ecdsa' : EcdsaError,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : DeferredMinterError });
  const ContractType = IDL.Variant({
    'Sell' : IDL.Null,
    'Financing' : IDL.Null,
  });
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
    'Principal' : IDL.Principal,
    'TextContent' : IDL.Text,
  });
  const RestrictionLevel = IDL.Variant({
    'Buyer' : IDL.Null,
    'Seller' : IDL.Null,
    'Agent' : IDL.Null,
  });
  const RestrictedProperty = IDL.Record({
    'value' : GenericValue,
    'access_list' : IDL.Vec(RestrictionLevel),
  });
  const Seller = IDL.Record({ 'quota' : IDL.Nat8, 'address' : IDL.Text });
  const ContractRegistration = IDL.Record({
    'value' : IDL.Nat64,
    'type' : ContractType,
    'restricted_properties' : IDL.Vec(IDL.Tuple(IDL.Text, RestrictedProperty)),
    'properties' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'deposit' : IDL.Nat64,
    'sellers' : IDL.Vec(Seller),
    'token_value' : IDL.Nat64,
    'expiration' : IDL.Text,
    'currency' : IDL.Text,
    'installments' : IDL.Nat64,
    'buyers' : IDL.Vec(IDL.Text),
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : DeferredMinterError });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Text,
    'Err' : DeferredMinterError,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'upgrade' : IDL.Opt(IDL.Bool),
    'status_code' : IDL.Nat16,
  });
  return IDL.Service({
    'admin_cycles' : IDL.Func([], [IDL.Nat], ['query']),
    'admin_ic_logs' : IDL.Func([Pagination], [Logs], ['query']),
    'admin_register_agency' : IDL.Func([IDL.Principal, Agency], [], []),
    'admin_remove_role' : IDL.Func([IDL.Principal, Role], [Result], []),
    'admin_set_allowed_currencies' : IDL.Func([IDL.Vec(IDL.Text)], [], []),
    'admin_set_custodians' : IDL.Func([IDL.Vec(IDL.Principal)], [Result], []),
    'admin_set_role' : IDL.Func([IDL.Principal, Role], [], []),
    'close_contract' : IDL.Func([IDL.Nat], [Result], []),
    'create_contract' : IDL.Func([ContractRegistration], [Result_1], []),
    'gas_station_set_gas_price' : IDL.Func([IDL.Nat64], [Result], []),
    'get_agencies' : IDL.Func([], [IDL.Vec(Agency)], ['query']),
    'get_eth_address' : IDL.Func([], [Result_2], []),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'remove_agency' : IDL.Func([IDL.Principal], [Result], []),
  });
};
export const init = ({ IDL }) => {
  const EcdsaKey = IDL.Variant({
    'Dfx' : IDL.Null,
    'Production' : IDL.Null,
    'Test' : IDL.Null,
  });
  const LogSettingsV2 = IDL.Record({
    'log_filter' : IDL.Text,
    'in_memory_records' : IDL.Nat64,
    'enable_console' : IDL.Bool,
    'max_record_length' : IDL.Nat64,
  });
  const DeferredMinterInitData = IDL.Record({
    'deferred_erc721' : IDL.Text,
    'evm_rpc_api' : IDL.Opt(IDL.Text),
    'allowed_currencies' : IDL.Vec(IDL.Text),
    'deferred_data' : IDL.Principal,
    'reward_pool' : IDL.Text,
    'custodians' : IDL.Vec(IDL.Principal),
    'chain_id' : IDL.Nat64,
    'evm_rpc' : IDL.Principal,
    'ecdsa_key' : EcdsaKey,
    'log_settings' : LogSettingsV2,
  });
  return [DeferredMinterInitData];
};
