use candid::{CandidType, Decode, Deserialize, Encode};
use did::StorableAccount;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::account::Account;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
/// Allowance key for mapping (from, spender) to allowance
pub struct AllowanceKey {
    pub balance_owner: StorableAccount,
    pub spender: StorableAccount,
}

impl AllowanceKey {
    pub fn new(balance_owner: Account, spender: Account) -> Self {
        Self {
            balance_owner: balance_owner.into(),
            spender: spender.into(),
        }
    }
}

impl From<Codec> for AllowanceKey {
    fn from(value: Codec) -> Self {
        Self {
            balance_owner: value.from.into(),
            spender: value.spender.into(),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, CandidType, Deserialize)]
struct Codec {
    from: Account,
    spender: Account,
}

impl Storable for AllowanceKey {
    const BOUND: Bound = Bound::Bounded {
        max_size: StorableAccount::BOUND.max_size() * 2,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Codec).unwrap().into()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let codec = Codec {
            from: self.balance_owner.0,
            spender: self.spender.0,
        };
        Encode!(&codec).unwrap().into()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice_account, bob_account};
    use crate::utils::caller;

    #[test]
    fn test_should_encode_and_decode_allowance_key() {
        let allowance_key = AllowanceKey {
            balance_owner: bob_account().into(),
            spender: alice_account().into(),
        };

        let encoded = allowance_key.to_bytes();
        let decoded = AllowanceKey::from_bytes(encoded);

        assert_eq!(allowance_key, decoded);
    }

    #[test]
    fn test_should_encode_and_decode_allowance_key_with_none() {
        let allowance_key = AllowanceKey {
            balance_owner: bob_account().into(),
            spender: Account {
                owner: caller(),
                subaccount: None,
            }
            .into(),
        };

        let encoded = allowance_key.to_bytes();
        let decoded = AllowanceKey::from_bytes(encoded);

        assert_eq!(allowance_key, decoded);
    }
}
