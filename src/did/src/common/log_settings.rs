use std::borrow::Cow;

use candid::{Decode, Encode};
use ic_log::LogSettingsV2;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;

#[derive(Debug, Default, Clone)]
pub struct StorableLogSettings(pub LogSettingsV2);

impl Storable for StorableLogSettings {
    const BOUND: Bound = Bound::Bounded {
        max_size: 512,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Encode!(&self.0).unwrap().into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let settings = Decode!(&bytes, LogSettingsV2).unwrap();

        Self(settings)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_encode_and_decode_settings() {
        let settings = LogSettingsV2 {
            enable_console: true,
            in_memory_records: 1024,
            max_record_length: 1024,
            log_filter: "debug".to_string(),
        };

        let storable = StorableLogSettings(settings.clone());
        let bytes = storable.to_bytes();
        let decoded = StorableLogSettings::from_bytes(bytes);

        assert_eq!(decoded.0, settings);
    }
}
