//! Types associated to the "Sell Contract" canister

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;

use crate::ID;

/// A sell contract for a building
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Contract {
    /// Contract ID
    pub id: ID,
    /// The contractor selling the building
    pub seller: Principal,
    /// Contract buyers. Those who must pay
    pub buyers: Vec<Principal>,
    /// Contract expiration date
    pub expiration: String,
    /// Tokens associated to the contract, by id
    pub tokens: Vec<ID>,
    /// $FLY reward for buying a Token
    pub fly_reward: u64,
    /// Data associated to the building
    pub building: BuildingData,
}

impl Storable for Contract {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

/// A Non fungible token related to an installment of a contract
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Token {
    /// Unique identifier of the token
    pub id: String,
    /// Contract id
    pub contract_id: String,
    /// Token owner
    pub owner: Principal,
    /// Value of the single token ($ICP)
    pub value: u64,
}

impl Storable for Token {
    const BOUND: Bound = Bound::Bounded {
        max_size: 36 + 36 + 29 + 8,
        is_fixed_size: true,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

/// Data associated to a building
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BuildingData {
    /// The city the building is located at
    pub city: String,
    /// FIAT currency value of the building
    pub fiat_value: u64,
}

impl Storable for BuildingData {
    const BOUND: Bound = Bound::Bounded {
        max_size: 256,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).unwrap()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_token() {
        let token = Token {
            id: "8bc80e74-4a42-4480-9c34-4d4993532a3b".to_string(),
            contract_id: "375b5279-1eba-44ce-98fc-9adc3520111c".to_string(),
            owner: Principal::from_text(
                "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
            )
            .unwrap(),
            value: 100,
        };
        let data = Encode!(&token).unwrap();
        let decoded_token = Decode!(&data, Token).unwrap();

        assert_eq!(token.id, decoded_token.id);
        assert_eq!(token.contract_id, decoded_token.contract_id);
        assert_eq!(token.owner, decoded_token.owner);
        assert_eq!(token.value, decoded_token.value);
    }

    #[test]
    fn test_should_encode_building_data() {
        let building_data = BuildingData {
            city: "Rome".to_string(),
            fiat_value: 250_000,
        };
        let data = Encode!(&building_data).unwrap();
        let decoded_building_data = Decode!(&data, BuildingData).unwrap();

        assert_eq!(building_data.city, decoded_building_data.city);
        assert_eq!(building_data.fiat_value, decoded_building_data.fiat_value);
    }

    #[test]
    fn test_should_encode_contract() {
        let contract = Contract {
            id: ID::random(),
            seller: Principal::from_text(
                "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
            )
            .unwrap(),
            buyers: vec![
                Principal::anonymous(),
                Principal::from_text(
                    "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
                )
                .unwrap(),
            ],
            expiration: "2021-12-31".to_string(),
            tokens: vec![ID::random(), ID::random()],
            fly_reward: 10,
            building: BuildingData {
                city: "Rome".to_string(),
                fiat_value: 250_000,
            },
        };
        let data = Encode!(&contract).unwrap();
        let decoded_contract = Decode!(&data, Contract).unwrap();

        assert_eq!(contract.id, decoded_contract.id);
        assert_eq!(contract.seller, decoded_contract.seller);
        assert_eq!(contract.buyers, decoded_contract.buyers);
        assert_eq!(contract.expiration, decoded_contract.expiration);
        assert_eq!(contract.tokens, decoded_contract.tokens);
        assert_eq!(contract.fly_reward, decoded_contract.fly_reward);
        assert_eq!(contract.building.city, decoded_contract.building.city);
        assert_eq!(
            contract.building.fiat_value,
            decoded_contract.building.fiat_value
        );
    }
}
