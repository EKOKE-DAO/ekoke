use candid::{Nat, Principal};
use did::sell_contract::BuildingData;
use did::ID;
use ic_cdk::api;
#[cfg(target_family = "wasm")]
use ic_cdk_macros::inspect_message;

use crate::app::SellContract;

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
        method if method.starts_with("admin_") => SellContract::inspect_is_custodian(),
        "set_logo" | "set_name" | "set_symbol" | "set_custodians" => {
            SellContract::inspect_is_custodian()
        }
        "register_contract" => {
            let (id, _, _, expiration, value, installments, _) = api::call::arg_data::<(
                ID,
                Principal,
                Vec<Principal>,
                String,
                u64,
                u64,
                BuildingData,
            )>();
            SellContract::inspect_register_contract(&id, value, installments, &expiration).is_ok()
        }
        "burn" => {
            let token_identifier = api::call::arg_data::<(Nat,)>().0;
            SellContract::inspect_burn(&token_identifier).is_ok()
        }
        "transfer_from" => {
            let (_, _, token_identifier) = api::call::arg_data::<(Principal, Principal, Nat)>();
            SellContract::inspect_is_owner_or_operator(&token_identifier).is_ok()
        }
        _ => false,
    };

    if check_result {
        api::call::accept_message();
    } else {
        ic_cdk::trap("Unauthorized");
    }
}
