use std::fmt::Display;
use std::str::FromStr;

use ic_stable_structures::Storable;

/// Represents a currency code with max length set to 8.
/// The currency code is encoded as a fixed-size array of 8 bytes. The currency code is padded with 0s on right.
///
/// e.g. "USD" is encoded as [85, 83, 68, 0, 0, 0, 0, 0]
pub struct Currency([u8; 8]);

impl Currency {
    /// Returns the length of the currency code.
    fn len(&self) -> usize {
        self.0.iter().position(|&x| x == 0).unwrap_or(8)
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.0[0..self.len()]).unwrap())
    }
}

impl FromStr for Currency {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 8 {
            return Err("invalid currency length".into());
        }

        // copy_from_slice requires the same length
        let mut bytes = [0; 8];
        bytes[..s.len()].copy_from_slice(s.as_bytes());
        Ok(Self(bytes))
    }
}

impl Storable for Currency {
    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Bounded {
            max_size: 8,
            is_fixed_size: true,
        };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut arr = [0; 8];
        arr.copy_from_slice(&bytes);
        Self(arr)
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(self.0.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency() {
        let currency = Currency::from_str("USD").unwrap();
        assert_eq!(currency.to_string(), "USD");
    }

    #[test]
    fn test_currency_encoding() {
        let currency = Currency::from_str("USD").unwrap();
        let bytes = currency.to_bytes();
        let decoded = Currency::from_bytes(bytes);
        assert_eq!(currency.0, decoded.0);
    }
}
