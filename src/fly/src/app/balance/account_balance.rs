use bytes::{Buf, BufMut, Bytes, BytesMut};
use did::fly::PicoFly;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Describes the balance of an account
pub struct Balance {
    pub amount: PicoFly,
}

impl From<PicoFly> for Balance {
    fn from(amount: PicoFly) -> Self {
        Self { amount }
    }
}

impl Storable for Balance {
    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut bytes = Bytes::from(bytes.to_vec());
        let amount = bytes.get_u64();

        Self { amount }
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = BytesMut::with_capacity(Self::BOUND.max_size() as usize);
        bytes.put_u64(self.amount);

        bytes.to_vec().into()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_and_decode_balance() {
        let balance = Balance {
            amount: 1_000_000_000_000 * 8_888_888,
        };

        let encoded = balance.to_bytes();
        let decoded = Balance::from_bytes(encoded);

        assert_eq!(balance, decoded);
    }
}
