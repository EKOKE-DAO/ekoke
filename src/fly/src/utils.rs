use candid::{Nat, Principal};
use icrc::icrc1::account::Subaccount;

/// Returns current time in nanoseconds
pub fn time() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        time.as_nanos() as u64
    }
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::time()
    }
}

/// Returns canister id
pub fn id() -> Principal {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Principal::from_text("lj532-6iaaa-aaaah-qcc7a-cai").unwrap()
    }
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::id()
    }
}

pub fn cycles() -> Nat {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Nat::from(30_000_000_000_u64)
    }
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::canister_balance().into()
    }
}

pub fn caller() -> Principal {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
            .unwrap()
    }
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::caller()
    }
}

/// Generates a random subaccount
pub fn random_subaccount() -> Subaccount {
    Subaccount::from([rand::random::<u8>(); 32])
}

/// Convert fly to picofly
pub fn fly_to_picofly(amount: u64) -> u64 {
    amount * 1_000_000_000_000
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_should_convert_fly_to_picofly() {
        assert_eq!(fly_to_picofly(1), 1_000_000_000_000);
        assert_eq!(fly_to_picofly(20), 20_000_000_000_000);
        assert_eq!(fly_to_picofly(300), 300_000_000_000_000);
    }
}
