use candid::{Decode, Encode};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::account::Account;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct StorableAccount(pub Account);

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

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice, bob_account};

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
