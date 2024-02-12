use did::ekoke::PicoEkoke;
use did::ID;
use ic_cdk::api;
#[cfg(target_family = "wasm")]
use ic_cdk_macros::inspect_message;
use icrc::icrc1::transfer::TransferArg;

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
        method if method.starts_with("admin_") => Inspect::inspect_is_admin(caller()),
        "get_contract_reward" => Inspect::inspect_is_deferred_canister(caller()),
        "send_reward" => {
            let contract_id: candid::Nat = api::call::arg_data::<(ID, PicoEkoke)>().0;

            Inspect::inspect_is_marketplace_canister(caller())
                && Inspect::inspect_pool_exists(&contract_id)
        }
        "icrc1_transfer" => {
            let transfer_arg = api::call::arg_data::<(TransferArg,)>().0;
            Inspect::inspect_transfer(&transfer_arg).is_ok()
        }
        "icrc2_approve" => {
            let args = api::call::arg_data::<(icrc::icrc2::approve::ApproveArgs,)>().0;
            Inspect::inspect_icrc2_approve(caller(), &args).is_ok()
        }
        "icrc2_transfer_from" => {
            let args = api::call::arg_data::<(icrc::icrc2::transfer_from::TransferFromArgs,)>().0;
            Inspect::inspect_icrc2_transfer_from(&args).is_ok()
        }

        _ => true,
    };

    if check_result {
        api::call::accept_message();
    } else {
        ic_cdk::trap("Bad request");
    }
}
