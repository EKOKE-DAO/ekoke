use candid::Principal;
use icrc::icrc1::account::{Account, Subaccount, DEFAULT_SUBACCOUNT};
use rand::Rng as _;

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

pub fn charlie() -> Principal {
    Principal::from_text("efv5g-kqaaa-aaaaq-aacaa-cai").unwrap()
}

pub fn charlie_account() -> Account {
    Account {
        owner: charlie(),
        subaccount: Some([
            0x21, 0xa9, 0x95, 0x49, 0xe7, 0x92, 0x90, 0x7c, 0x5e, 0x27, 0x5e, 0x54, 0x51, 0x06,
            0x8d, 0x4d, 0xdf, 0x4d, 0x43, 0xee, 0x8d, 0xca, 0xb4, 0x87, 0x56, 0x23, 0x1a, 0x8f,
            0xb7, 0x71, 0x31, 0x23,
        ]),
    }
}

pub fn random_alice_account() -> Account {
    Account {
        owner: alice(),
        subaccount: Some(random_subaccounts()),
    }
}

fn random_subaccounts() -> Subaccount {
    let mut rng = rand::thread_rng();
    let mut subaccount = [0u8; 32];
    rng.fill(&mut subaccount);
    subaccount
}
