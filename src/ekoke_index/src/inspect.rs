use ic_cdk::api;
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
        "commit" => Inspect::inspect_is_archive_canister(caller()),
        _ => true,
    };

    if check_result {
        api::call::accept_message();
    } else {
        ic_cdk::trap("Bad request");
    }
}
