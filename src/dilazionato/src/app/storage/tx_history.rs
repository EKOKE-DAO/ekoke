use candid::Nat;
use did::dilazionato::Token;
use dip721::{GenericValue, TxEvent};

use super::TX_HISTORY;

pub struct TxHistory;

impl TxHistory {
    /// Get transaction by id
    pub fn get_transaction_by_id(tx_id: Nat) -> Option<TxEvent> {
        TX_HISTORY
            .with_borrow(|tx_history| tx_history.get(&tx_id.into()))
            .map(|event| event.0)
    }

    /// Get transaction count
    pub fn count() -> u64 {
        TX_HISTORY.with_borrow(|tx_history| tx_history.len())
    }

    /// Register a token mint
    pub(super) fn register_token_mint(token: &Token) {
        let event = TxEvent {
            caller: crate::utils::caller(),
            details: vec![
                (
                    "token_id".to_string(),
                    GenericValue::NatContent(token.id.clone()),
                ),
                (
                    "contract_id".to_string(),
                    GenericValue::TextContent(token.contract_id.to_string()),
                ),
                (
                    "owner".to_string(),
                    GenericValue::Principal(token.owner.unwrap()),
                ),
                (
                    "minted_at".to_string(),
                    GenericValue::Nat64Content(token.minted_at),
                ),
            ],
            operation: "mint".to_string(),
            time: crate::utils::time(),
        };
        let id = Self::next_id();
        TX_HISTORY.with_borrow_mut(|tx_history| {
            tx_history.insert(id.into(), event.into());
        });
    }

    pub(super) fn register_token_burn(token: &Token) -> Nat {
        let event = TxEvent {
            caller: crate::utils::caller(),
            details: vec![
                (
                    "token_id".to_string(),
                    GenericValue::NatContent(token.id.clone()),
                ),
                (
                    "contract_id".to_string(),
                    GenericValue::TextContent(token.contract_id.to_string()),
                ),
                (
                    "burned_by".to_string(),
                    GenericValue::Principal(token.burned_by.unwrap()),
                ),
                (
                    "burned_at".to_string(),
                    GenericValue::Nat64Content(token.burned_at.unwrap()),
                ),
            ],
            operation: "burn".to_string(),
            time: crate::utils::time(),
        };
        let id = Self::next_id();
        TX_HISTORY.with_borrow_mut(|tx_history| {
            tx_history.insert(id.clone().into(), event.into());
        });

        id
    }

    pub(super) fn register_transfer(token: &Token) -> Nat {
        let event = TxEvent {
            caller: crate::utils::caller(),
            details: vec![
                (
                    "token_id".to_string(),
                    GenericValue::NatContent(token.id.clone()),
                ),
                (
                    "contract_id".to_string(),
                    GenericValue::TextContent(token.contract_id.to_string()),
                ),
                (
                    "transferred_by".to_string(),
                    GenericValue::Principal(token.transferred_by.unwrap()),
                ),
                (
                    "transferred_at".to_string(),
                    GenericValue::Nat64Content(token.transferred_at.unwrap()),
                ),
            ],
            operation: "transfer".to_string(),
            time: crate::utils::time(),
        };
        let id = Self::next_id();
        TX_HISTORY.with_borrow_mut(|tx_history| {
            tx_history.insert(id.clone().into(), event.into());
        });

        id
    }

    /// get next transaction id
    fn next_id() -> Nat {
        TX_HISTORY.with_borrow(|tx_history| tx_history.len()).into()
    }
}

#[cfg(test)]
mod test {

    use candid::Principal;
    use did::dilazionato::Token;
    use did::ID;
    use dip721::TokenIdentifier;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_insert_transactions() {
        let token = Token {
            id: TokenIdentifier::from(1),
            contract_id: ID::from(1),
            owner: Some(
                Principal::from_text(
                    "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
                )
                .unwrap(),
            ),
            transferred_at: None,
            transferred_by: None,
            approved_at: None,
            approved_by: None,
            burned_at: None,
            burned_by: None,
            minted_at: 0,
            value: 0,
            operator: None,
            is_burned: false,
            minted_by: Principal::anonymous(),
        };
        TxHistory::register_token_mint(&token);
        let tx = TxHistory::get_transaction_by_id(0.into()).unwrap();
        assert_eq!(tx.operation, "mint");
        assert_eq!(tx.caller, crate::utils::caller());
        assert_eq!(tx.details.len(), 4);
        assert_eq!(
            tx.details[0],
            (
                "token_id".to_string(),
                GenericValue::NatContent(token.id.clone())
            )
        );
        assert_eq!(
            tx.details[1],
            (
                "contract_id".to_string(),
                GenericValue::TextContent(token.contract_id.to_string())
            )
        );
        assert_eq!(
            tx.details[2],
            (
                "owner".to_string(),
                GenericValue::Principal(token.owner.unwrap())
            )
        );
        assert_eq!(
            tx.details[3],
            (
                "minted_at".to_string(),
                GenericValue::Nat64Content(token.minted_at)
            )
        );
        assert_eq!(TxHistory::count(), 1);
        assert_eq!(TxHistory::next_id(), 1);
    }
}
