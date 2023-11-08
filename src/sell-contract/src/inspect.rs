use candid::Nat;
use ic_cdk::api;
#[cfg(target_family = "wasm")]
use ic_cdk_macros::inspect_message;

use crate::app::SellContract;

/// NOTE: inspect is disabled for non-wasm targets because without it we are getting a weird compilation error
/// in CI:
/// > multiple definition of `canister_inspect_message'
#[cfg(target_family = "wasm")]
#[ic_exports::ic_cdk_macros::inspect_message]
fn inspect_messages() {
    inspect_message_impl()
}

#[allow(dead_code)]
fn inspect_message_impl() {
    let method = api::call::method_name();

    let check_result = match method.as_str() {
        method if method.starts_with("admin_") => SellContract::inspect_is_custodian(),
        "set_logo" | "set_name" | "set_symbol" | "set_custodians" => {
            SellContract::inspect_is_custodian()
        }
        "burn" => {
            let token_identifier = api::call::arg_data::<(Nat,)>().0;
            SellContract::inspect_burn(&token_identifier).is_ok()
        }
        _ => false,
    };

    if check_result {
        api::call::accept_message();
    } else {
        ic_cdk::trap("Unauthorized");
    }
}
