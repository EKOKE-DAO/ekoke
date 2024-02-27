use candid::Principal;

pub fn ekoke_ledger_canister() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}

pub fn ekoke_liquidity_pool_canister() -> Principal {
    Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap()
}

pub fn deferred_canister() -> Principal {
    Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap()
}
