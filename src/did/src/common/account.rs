use candid::{Decode, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::account::Account;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct StorableAccount(pub Account);

impl StorableAccount {
    pub fn is_anonymous(&self) -> bool {
        self.0.owner == Principal::anonymous()
    }
}

impl From<Account> for StorableAccount {
    fn from(value: Account) -> Self {
        Self(value)
    }
}

impl Storable for StorableAccount {
    const BOUND: Bound = Bound::Bounded {
        max_size: 128, // principal + 32 bytes of subaccount
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Account).unwrap().into()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self.0).unwrap().into()
    }
}

#[cfg(test)]
mod test {

    use candid::Principal;
    use pretty_assertions::assert_eq;

    use super::*;

    pub fn alice() -> Principal {
        Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai").unwrap()
    }

    pub fn bob() -> Principal {
        Principal::from_text("bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe")
            .unwrap()
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

    #[test]
    fn test_should_encode_and_decode_account() {
        let account = bob_account();
        let storable_account = StorableAccount(account.clone());
        let bytes = storable_account.to_bytes();
        let decoded_account = StorableAccount::from_bytes(bytes);
        assert_eq!(decoded_account, storable_account);

        let account = Account {
            owner: alice(),
            subaccount: None,
        };
        let storable_account = StorableAccount(account.clone());
        let bytes = storable_account.to_bytes();
        let decoded_account = StorableAccount::from_bytes(bytes);
        assert_eq!(decoded_account, storable_account);
    }
}
