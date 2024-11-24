export const idlFactory = ({ IDL }) => {
  const GenericValue = IDL.Rec();
  const DeferredDataInitData = IDL.Record({ 'minter' : IDL.Principal });
  const ConfigurationError = IDL.Variant({
    'AnonymousOwner' : IDL.Null,
    'AnonymousMinter' : IDL.Null,
  });
  const ContractError = IDL.Variant({
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
    'Configuration' : ConfigurationError,
    'Contract' : ContractError,
    'InvalidSignature' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'StorageError' : IDL.Null,
    'CanisterCall' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : DeferredDataError });
  const ContractType = IDL.Variant({
    'Sell' : IDL.Null,
    'Financing' : IDL.Null,
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
  GenericValue.fill(
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
      'NestedContent' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
      'Principal' : IDL.Principal,
      'TextContent' : IDL.Text,
    })
  );
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
  const Contract = IDL.Record({
    'id' : IDL.Nat,
    'closed' : IDL.Bool,
    'value' : IDL.Nat64,
    'type' : ContractType,
    'agency' : IDL.Opt(Agency),
    'restricted_properties' : IDL.Vec(IDL.Tuple(IDL.Text, RestrictedProperty)),
    'properties' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'deposit' : IDL.Nat64,
    'sellers' : IDL.Vec(Seller),
    'expiration' : IDL.Text,
    'currency' : IDL.Text,
    'installments' : IDL.Nat64,
    'buyers' : IDL.Vec(IDL.Text),
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
    'admin_set_minter' : IDL.Func([IDL.Principal], [Result], []),
    'get_contract' : IDL.Func([IDL.Nat], [IDL.Opt(Contract)], ['query']),
    'get_contracts' : IDL.Func([], [IDL.Vec(IDL.Nat)], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'minter_close_contract' : IDL.Func([IDL.Nat], [Result], []),
    'minter_create_contract' : IDL.Func([Contract], [Result], []),
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
  });
};
export const init = ({ IDL }) => {
  const DeferredDataInitData = IDL.Record({ 'minter' : IDL.Principal });
  return [DeferredDataInitData];
};
