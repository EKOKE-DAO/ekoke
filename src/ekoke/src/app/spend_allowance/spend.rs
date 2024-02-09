use bytes::{Buf as _, BufMut as _, Bytes, BytesMut};
use candid::Nat;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use icrc::icrc1::transfer::Memo;
use icrc::icrc2::approve::ApproveArgs;
use num_bigint::BigUint;

/// Storable spend allowance type
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Spend {
    /// spenable amount
    pub amount: Nat,
    /// If the expected_allowance field is set, it's equal to the current allowance for the spender.
    pub expected_allowance: Option<Nat>,
    pub expires_at: Option<u64>,
    pub fee: Option<Nat>,
    pub memo: Option<Memo>,
    pub created_at_time: u64,
}

impl From<ApproveArgs> for Spend {
    fn from(value: ApproveArgs) -> Self {
        Self {
            amount: value.amount,
            expected_allowance: value.expected_allowance,
            expires_at: value.expires_at,
            fee: value.fee,
            memo: value.memo,
            created_at_time: value.created_at_time.unwrap_or_else(crate::utils::time),
        }
    }
}

impl Storable for Spend {
    const BOUND: Bound = Bound::Bounded {
        max_size: 256,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut bytes = Bytes::from(bytes.to_vec());
        let amount_len = bytes.get_u8() as usize;
        let amount = BigUint::from_bytes_be(&bytes.slice(..amount_len)).into();
        bytes.advance(amount_len);

        let expected_allowance_len = bytes.get_u8() as usize;
        let expected_allowance = if expected_allowance_len > 0 {
            let expected_allowance =
                BigUint::from_bytes_be(&bytes.slice(..expected_allowance_len)).into();
            bytes.advance(expected_allowance_len);

            Some(expected_allowance)
        } else {
            None
        };

        let expires_at = if bytes.get_u8() == 1 {
            let expires_at = bytes.get_u64();

            Some(expires_at)
        } else {
            None
        };

        let fee_len = bytes.get_u8() as usize;
        let fee = if fee_len > 0 {
            let fee = BigUint::from_bytes_be(&bytes.slice(..fee_len)).into();
            bytes.advance(fee_len);

            Some(fee)
        } else {
            None
        };

        let memo_len = bytes.get_u8() as usize;
        let memo = if memo_len > 0 {
            let memo = Memo::from(bytes.slice(..memo_len).to_vec());
            bytes.advance(memo_len);

            Some(memo)
        } else {
            None
        };

        let created_at_time = bytes.get_u64();

        Self {
            amount,
            expected_allowance,
            expires_at,
            fee,
            memo,
            created_at_time,
        }
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut buffer = BytesMut::with_capacity(Self::BOUND.max_size() as usize);

        // amount
        let amount_bytes = self.amount.0.to_bytes_be();
        buffer.put_u8(amount_bytes.len() as u8);
        buffer.put(self.amount.0.to_bytes_be().as_slice());

        // expected allowance
        let expected_allowance = self
            .expected_allowance
            .as_ref()
            .map(|x| x.0.to_bytes_be())
            .unwrap_or_default();
        buffer.put_u8(expected_allowance.len() as u8); // if zero, don't read expected allowance
        buffer.put(expected_allowance.as_slice());

        // expires at
        buffer.put_u8(self.expires_at.is_some() as u8); // 1 if expires_at is some
        if let Some(expires_at) = self.expires_at {
            buffer.put_u64(expires_at);
        }

        // fee
        let fee = self
            .fee
            .as_ref()
            .map(|x| x.0.to_bytes_be())
            .unwrap_or_default();
        buffer.put_u8(fee.len() as u8); // if zero, don't read expected allowance
        buffer.put(fee.as_slice());

        // memo
        let memo = self.memo.as_ref().map(|x| x.0.to_vec()).unwrap_or_default();
        buffer.put_u8(memo.len() as u8); // if zero, don't read expected allowance
        buffer.put(memo.as_slice());

        // created_at_time
        buffer.put_u64(self.created_at_time);

        buffer.to_vec().into()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_and_decode_with_options_none() {
        let spend = Spend {
            amount: 100_000_000_000_000__u64.into(),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: crate::utils::time(),
        };

        let encoded = spend.to_bytes();
        let decoded = Spend::from_bytes(encoded);

        assert_eq!(spend, decoded);
    }

    #[test]
    fn test_should_encode_and_decode_with_options_some() {
        let spend = Spend {
            amount: 100_000_000_000_000__u64.into(),
            expected_allowance: Some(100_000_000_000_000__u64.into()),
            expires_at: Some(crate::utils::time()),
            fee: Some(100_000_000_000_000__u64.into()),
            memo: Some(Memo::from(vec![1; 48])),
            created_at_time: crate::utils::time(),
        };

        let encoded = spend.to_bytes();
        let decoded = Spend::from_bytes(encoded);

        assert_eq!(spend, decoded);
    }
}
