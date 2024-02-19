use bytes::{Buf, BufMut, Bytes, BytesMut};
use did::ekoke::PicoEkoke;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Describes the balance of an account
pub struct Balance {
    pub amount: PicoEkoke,
}

impl From<PicoEkoke> for Balance {
    fn from(amount: PicoEkoke) -> Self {
        Self { amount }
    }
}

impl Storable for Balance {
    const BOUND: Bound = Bound::Bounded {
        max_size: 64,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut bytes = Bytes::from(bytes.to_vec());

        let amount_len = bytes.get_u8();

        bytes.slice(..amount_len as usize);
        let amount = BigUint::from_bytes_be(&bytes.slice(..amount_len as usize)).into();

        Self { amount }
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = BytesMut::with_capacity(Self::BOUND.max_size() as usize);
        let amount_bytes = self.amount.0.to_bytes_be();
        bytes.put_u8(amount_bytes.len() as u8);
        bytes.put(self.amount.0.to_bytes_be().as_slice());

        bytes.to_vec().into()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::ekoke_to_picoekoke;

    #[test]
    fn test_should_encode_and_decode_balance() {
        let balance = Balance {
            amount: ekoke_to_picoekoke(8_888_888),
        };

        let encoded = balance.to_bytes();
        let decoded = Balance::from_bytes(encoded);

        assert_eq!(balance, decoded);
    }
}
