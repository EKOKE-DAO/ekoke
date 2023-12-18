use candid::{Nat, Principal};
use did::deferred::ContractRegistration;
use did::ID;
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
        method if method.starts_with("admin_") => Inspect::inspect_is_custodian(caller()),
        "set_logo" | "set_name" | "set_symbol" | "set_custodians" => {
            Inspect::inspect_is_custodian(caller())
        }
        "seller_increment_contract_value" => {
            let (id, _, _) = api::call::arg_data::<(ID, u64, u64)>();
            Inspect::inspect_is_buyer(caller(), id).is_ok()
        }
        "update_contract_property" => {
            let (id, key, _) = api::call::arg_data::<(ID, String, u64)>();
            Inspect::inspect_update_contract_property(caller(), &id, &key).is_ok()
        }
        "register_contract" => {
            let data = api::call::arg_data::<(ContractRegistration,)>().0;
            Inspect::inspect_register_contract(
                caller(),
                data.value,
                &data.sellers,
                data.installments,
            )
            .is_ok()
        }
        "close_contract" => {
            let id = api::call::arg_data::<(ID,)>().0;
            Inspect::inspect_close_contract(caller(), id).is_ok()
        }
        "burn" => {
            let token_identifier = api::call::arg_data::<(Nat,)>().0;
            Inspect::inspect_burn(caller(), &token_identifier).is_ok()
        }
        "transfer_from" => {
            let (_, _, token_identifier) = api::call::arg_data::<(Principal, Principal, Nat)>();
            Inspect::inspect_is_owner_or_operator(caller(), &token_identifier).is_ok()
        }
        _ => true,
    };

    if check_result {
        api::call::accept_message();
    } else {
        ic_cdk::trap("Bad request");
    }
}
