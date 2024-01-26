use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::Memo;

use super::PicoFly;

#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub struct Transaction {
    pub from: Account,
    pub to: Account,
    pub amount: PicoFly,
    pub fee: PicoFly,
    pub memo: Option<Memo>,
    pub created_at: u64,
}

impl Storable for Transaction {
    const BOUND: Bound = Bound::Bounded {
        max_size: 256,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Transaction).unwrap()
    }
}

#[cfg(test)]
mod test {

    use candid::Principal;
    use icrc::icrc1::account::Account;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_transaction() {
        let tx = Transaction {
            from: Account {
                owner: Principal::management_canister(),
                subaccount: Some([1u8; 32]),
            },
            to: Account {
                owner: Principal::management_canister(),
                subaccount: None,
            },
            amount: 100_u64.into(),
            fee: 1_u64.into(),
            memo: None,
            created_at: 0,
        };

        let data = tx.to_bytes();
        let decoded_tx = Transaction::from_bytes(data);
        assert_eq!(tx, decoded_tx);
    }
}
