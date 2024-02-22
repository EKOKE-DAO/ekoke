use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;

/// Ethereum network
#[derive(Clone, Copy, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub enum EthNetwork {
    /// Main net
    Ethereum,
    /// Goerli testnet
    Goerli,
    /// Sepolia testnet
    Sepolia,
}

impl EthNetwork {
    pub fn chain_id(&self) -> u64 {
        match self {
            EthNetwork::Ethereum => 1,
            EthNetwork::Goerli => 5,
            EthNetwork::Sepolia => 11155111,
        }
    }
}

impl Storable for EthNetwork {
    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: true,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(&self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, EthNetwork).unwrap()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_network() {
        let network = EthNetwork::Ethereum;

        let data = network.to_bytes();
        let decoded = EthNetwork::from_bytes(data);
        assert_eq!(network, decoded);
    }
}
