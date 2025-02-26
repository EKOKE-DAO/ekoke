use did::deferred::ContractRegistration;
use ic_cdk::api;
use ic_cdk::api::call::ArgDecoderConfig;
#[cfg(target_family = "wasm")]
use ic_cdk_macros::inspect_message;

use crate::app::Inspect;
use crate::utils::caller;

/// NOTE: inspect is disabled for non-wasm targets because without it we are getting a weird compilation error
/// in CI:
/// > multiple definition of `canister_inspect_message'
#[cfg(target_family = "wasm")]
#[inspect_message]
fn inspect_messages() {
    inspect_message_impl()
}

#[allow(dead_code)]
fn inspect_message_impl() {
    let method = api::call::method_name();

    let check_result = match method.as_str() {
        method if method.starts_with("admin_") => Inspect::inspect_is_custodian(caller()),
        method if method.starts_with("gas_station_") => Inspect::inspect_is_gas_station(caller()),
        "create_contract" => {
            let data =
                api::call::arg_data::<(ContractRegistration,)>(ArgDecoderConfig::default()).0;
            Inspect::inspect_register_contract(caller(), &data).is_ok()
        }
        "close_contract" => {
            Inspect::inspect_is_custodian(caller()) || Inspect::inspect_is_custodian(caller())
        }
        "create_real_estate" => Inspect::inspect_is_agent(caller()),
        "update_real_estate" => Inspect::inspect_is_agent(caller()),
        "delete_real_estate" => Inspect::inspect_is_agent(caller()),
        _ => true,
    };

    if check_result {
        api::call::accept_message();
    } else {
        ic_cdk::trap(&format!("Unauthorized call to {}", method));
    }
}
