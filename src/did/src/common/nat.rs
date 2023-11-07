use std::borrow::Cow;

use candid::Nat;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use num_bigint::BigUint;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct StorableNat(pub Nat);

impl StorableNat {
    /// as Nat
    pub fn as_nat(&self) -> &Nat {
        &self.0
    }
}

impl From<Nat> for StorableNat {
    fn from(value: Nat) -> Self {
        Self(value)
    }
}

impl Storable for StorableNat {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        let big_uint = &self.0 .0;
        big_uint.to_bytes_be().into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        let big_uint = BigUint::from_bytes_be(bytes.as_ref());
        Self(Nat::from(big_uint))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 24,
        is_fixed_size: false,
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_nat_roundtrip() {
        let value = Nat::from(8_888_888);
        let storable = StorableNat::from(value.clone());
        let bytes = storable.to_bytes();
        let storable_actual = StorableNat::from_bytes(bytes);
        assert_eq!(storable_actual, storable);
    }
}
