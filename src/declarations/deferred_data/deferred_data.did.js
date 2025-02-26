export const idlFactory = ({ IDL }) => {
  const LogSettingsV2 = IDL.Record({
    'log_filter' : IDL.Text,
    'in_memory_records' : IDL.Nat64,
    'enable_console' : IDL.Bool,
    'max_record_length' : IDL.Nat64,
  });
  const DeferredDataInitData = IDL.Record({
    'minter' : IDL.Principal,
    'log_settings' : LogSettingsV2,
  });
  const Pagination = IDL.Record({ 'count' : IDL.Nat64, 'offset' : IDL.Nat64 });
  const Log = IDL.Record({ 'log' : IDL.Text, 'offset' : IDL.Nat64 });
  const Logs = IDL.Record({
    'logs' : IDL.Vec(Log),
    'all_logs_count' : IDL.Nat64,
  });
  const ConfigurationError = IDL.Variant({
    'AnonymousOwner' : IDL.Null,
    'AnonymousMinter' : IDL.Null,
  });
  const ContractError = IDL.Variant({
    'DocumentNotFound' : IDL.Nat64,
    'ContractNotFound' : IDL.Nat,
    'DocumentSizeMismatch' : IDL.Tuple(IDL.Nat64, IDL.Nat64),
    'BadContractProperty' : IDL.Null,
  });
  const RealEstateError = IDL.Variant({ 'NotFound' : IDL.Nat });
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
    'Configuration' : ConfigurationError,
    'Contract' : ContractError,
    'RealEstate' : RealEstateError,
    'InvalidSignature' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'StorageError' : IDL.Null,
    'CanisterCall' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : DeferredDataError });
  const RestrictionLevel = IDL.Variant({
    'Buyer' : IDL.Null,
    'Public' : IDL.Null,
    'Seller' : IDL.Null,
    'Agent' : IDL.Null,
  });
  const ContractDocument = IDL.Record({
    'name' : IDL.Text,
    'size' : IDL.Nat64,
    'mime_type' : IDL.Text,
    'access_list' : IDL.Vec(RestrictionLevel),
  });
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
  const RestrictedProperty = IDL.Record({
    'value' : GenericValue,
    'access_list' : IDL.Vec(RestrictionLevel),
  });
  const Seller = IDL.Record({ 'quota' : IDL.Nat8, 'address' : IDL.Text });
  const Contract = IDL.Record({
    'id' : IDL.Nat,
    'closed' : IDL.Bool,
    'documents' : IDL.Vec(IDL.Tuple(IDL.Nat64, ContractDocument)),
    'value' : IDL.Nat64,
    'type' : ContractType,
    'agency' : IDL.Principal,
    'restricted_properties' : IDL.Vec(IDL.Tuple(IDL.Text, RestrictedProperty)),
    'properties' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'deposit' : IDL.Nat64,
    'sellers' : IDL.Vec(Seller),
    'expiration' : IDL.Text,
    'currency' : IDL.Text,
    'real_estate' : IDL.Nat,
    'installments' : IDL.Nat64,
    'buyers' : IDL.Vec(IDL.Text),
  });
  const ContractDocumentData = IDL.Record({
    'data' : IDL.Vec(IDL.Nat8),
    'name' : IDL.Text,
    'mime_type' : IDL.Text,
  });
  const Result_1 = IDL.Variant({
    'Ok' : ContractDocumentData,
    'Err' : DeferredDataError,
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
  const RealEstate = IDL.Record({
    'region' : IDL.Opt(IDL.Text),
    'latitude' : IDL.Opt(IDL.Float64),
    'energy_class' : IDL.Opt(IDL.Text),
    'zip_code' : IDL.Opt(IDL.Text),
    'deleted' : IDL.Bool,
    'square_meters' : IDL.Opt(IDL.Nat64),
    'country' : IDL.Opt(IDL.Text),
    'bedrooms' : IDL.Opt(IDL.Nat64),
    'floors' : IDL.Opt(IDL.Nat64),
    'city' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'pool' : IDL.Opt(IDL.Bool),
    'zone' : IDL.Opt(IDL.Text),
    'garage' : IDL.Opt(IDL.Bool),
    'garden' : IDL.Opt(IDL.Bool),
    'agency' : IDL.Principal,
    'continent' : IDL.Opt(Continent),
    'description' : IDL.Text,
    'longitude' : IDL.Opt(IDL.Float64),
    'address' : IDL.Opt(IDL.Text),
    'elevator' : IDL.Opt(IDL.Bool),
    'youtube' : IDL.Opt(IDL.Text),
    'image' : IDL.Opt(IDL.Text),
    'balconies' : IDL.Opt(IDL.Nat64),
    'bathrooms' : IDL.Opt(IDL.Nat64),
    'year_of_construction' : IDL.Opt(IDL.Nat64),
    'parking' : IDL.Opt(IDL.Bool),
    'rooms' : IDL.Opt(IDL.Nat64),
  });
  const Result_2 = IDL.Variant({
    'Ok' : RealEstate,
    'Err' : DeferredDataError,
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
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : DeferredDataError });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : DeferredDataError });
  return IDL.Service({
    'admin_cycles' : IDL.Func([], [IDL.Nat], ['query']),
    'admin_ic_logs' : IDL.Func([Pagination], [Logs], ['query']),
    'admin_set_minter' : IDL.Func([IDL.Principal], [Result], []),
    'get_contract' : IDL.Func([IDL.Nat], [IDL.Opt(Contract)], ['query']),
    'get_contract_document' : IDL.Func(
        [IDL.Nat, IDL.Nat64],
        [Result_1],
        ['query'],
      ),
    'get_contracts' : IDL.Func([], [IDL.Vec(IDL.Nat)], ['query']),
    'get_real_estate' : IDL.Func([IDL.Nat], [Result_2], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'minter_close_contract' : IDL.Func([IDL.Nat], [Result], []),
    'minter_create_contract' : IDL.Func([Contract], [Result], []),
    'minter_create_real_estate' : IDL.Func([RealEstate], [Result_3], []),
    'minter_delete_real_estate' : IDL.Func([IDL.Nat], [Result], []),
    'minter_update_real_estate' : IDL.Func([IDL.Nat, RealEstate], [Result], []),
    'update_contract_property' : IDL.Func(
        [IDL.Nat, IDL.Text, GenericValue],
        [Result],
        [],
      ),
    'update_restricted_contract_property' : IDL.Func(
        [IDL.Nat, IDL.Text, RestrictedProperty],
        [Result],
        [],
      ),
    'upload_contract_document' : IDL.Func(
        [IDL.Nat, ContractDocument, IDL.Vec(IDL.Nat8)],
        [Result_4],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const LogSettingsV2 = IDL.Record({
    'log_filter' : IDL.Text,
    'in_memory_records' : IDL.Nat64,
    'enable_console' : IDL.Bool,
    'max_record_length' : IDL.Nat64,
  });
  const DeferredDataInitData = IDL.Record({
    'minter' : IDL.Principal,
    'log_settings' : LogSettingsV2,
  });
  return [DeferredDataInitData];
};
