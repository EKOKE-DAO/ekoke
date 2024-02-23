use candid::{Nat, Principal};
use did::ekoke::Ekoke;
use icrc::icrc1::account::Account;

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

/// Convert ekoke to picoekoke
pub fn ekoke_to_picoekoke(amount: u64) -> Ekoke {
    let amount = Nat::from(amount);
    let multiplier = Nat::from(1_000_000_000_000_u64);
    amount * multiplier
}

mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_convert_ekoke_to_picoekoke() {
        assert_eq!(ekoke_to_picoekoke(1), 1_000_000_000_000_u64);
        assert_eq!(ekoke_to_picoekoke(20), 20_000_000_000_000_u64);
        assert_eq!(ekoke_to_picoekoke(300), 300_000_000_000_000_u64);
    }
}
