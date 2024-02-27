use candid::{CandidType, Decode, Deserialize, Encode, Nat};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::Memo;

use super::TxId;

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct TransactionWithId {
    pub id: TxId,
    pub transaction: Transaction,
}

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct Transaction {
    pub kind: String,
    pub mint: Option<Mint>,
    pub burn: Option<Burn>,
    pub transfer: Option<Transfer>,
    pub approve: Option<Approve>,
    pub timestamp: u64,
}

impl Storable for Transaction {
    const BOUND: Bound = Bound::Bounded {
        max_size: 4096,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Transaction).unwrap()
    }
}

impl Transaction {
    pub fn from(&self) -> Option<Account> {
        if let Some(burn) = &self.burn {
            Some(burn.from)
        } else if let Some(transfer) = &self.transfer {
            Some(transfer.from)
        } else {
            self.approve.as_ref().map(|approve| approve.from)
        }
    }

    pub fn to(&self) -> Option<Account> {
        if let Some(transfer) = &self.transfer {
            Some(transfer.to)
        } else {
            self.mint.as_ref().map(|mint| mint.to)
        }
    }

    pub fn spender(&self) -> Option<Account> {
        if let Some(transfer) = &self.transfer {
            transfer.spender
        } else if let Some(approve) = &self.approve {
            approve.spender
        } else if let Some(burn) = &self.burn {
            burn.spender
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct Mint {
    pub amount: Nat,
    pub to: Account,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct Burn {
    pub amount: Nat,
    pub from: Account,
    pub spender: Option<Account>,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct Transfer {
    pub amount: Nat,
    pub from: Account,
    pub to: Account,
    pub spender: Option<Account>,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
    pub fee: Option<Nat>,
}

#[derive(Debug, Clone, PartialEq, CandidType, Deserialize)]
pub struct Approve {
    pub amount: Nat,
    pub from: Account,
    pub spender: Option<Account>,
    pub expected_allowance: Option<Nat>,
    pub expires_at: Option<u64>,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
    pub fee: Option<Nat>,
}

#[cfg(test)]
mod test {
    use candid::Principal;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_store_transaction_with_id() {
        let tx = Transaction {
            kind: "transfer".to_string(),
            mint: None,
            burn: None,
            transfer: Some(Transfer {
                amount: 100_u64.into(),
                from: Account {
                    owner: Principal::management_canister(),
                    subaccount: Some([1u8; 32]),
                },
                to: Account {
                    owner: Principal::management_canister(),
                    subaccount: None,
                },
                spender: None,
                memo: None,
                created_at_time: None,
                fee: None,
            }),
            approve: None,
            timestamp: 0,
        };

        let data = tx.to_bytes();
        let decoded_tx = Transaction::from_bytes(data);
        assert_eq!(tx, decoded_tx);
    }
}
