use candid::{Nat, Principal};
use did::fly::PicoFly;
use icrc::icrc1::account::{Account, DEFAULT_SUBACCOUNT};

use crate::utils::caller;

pub fn alice() -> Principal {
    Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()
}

pub fn alice_account() -> Account {
    Account {
        owner: alice(),
        subaccount: Some(*DEFAULT_SUBACCOUNT),
    }
}

pub fn bob() -> Principal {
    Principal::from_text("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe").unwrap()
}

pub fn bob_account() -> Account {
    Account {
        owner: bob(),
        subaccount: Some([
            0x21, 0xa9, 0x95, 0x49, 0xe7, 0x92, 0x90, 0x7c, 0x5e, 0x27, 0x5e, 0x54, 0x51, 0x06,
            0x8d, 0x4d, 0xdf, 0x4d, 0x43, 0xee, 0x8d, 0xca, 0xb4, 0x87, 0x56, 0x23, 0x1a, 0x8f,
            0xb7, 0x71, 0x31, 0x23,
        ]),
    }
}

pub fn caller_account() -> Account {
    Account {
        owner: caller(),
        subaccount: Some(*DEFAULT_SUBACCOUNT),
    }
}

pub fn fly_canister() -> Principal {
    Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap()
}

pub fn deferred_canister() -> Principal {
    Principal::from_text("r7inp-6aaaa-aaaaa-aaabq-cai").unwrap()
}
