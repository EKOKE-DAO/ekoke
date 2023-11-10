use candid::Principal;
use did::{dilazionato::Token, ID};
use dip721::TokenIdentifier;

pub fn mock_token(id: u64, contract_id: u64) -> Token {
    Token {
        id: TokenIdentifier::from(id),
        contract_id: ID::from(contract_id),
        owner: Some(
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap(),
        ),
        transferred_at: None,
        transferred_by: None,
        approved_at: None,
        approved_by: None,
        mfly_reward: 4000,
        burned_at: None,
        burned_by: None,
        minted_at: 0,
        value: 100,
        operator: None,
        is_burned: false,
        minted_by: Principal::anonymous(),
    }
}
