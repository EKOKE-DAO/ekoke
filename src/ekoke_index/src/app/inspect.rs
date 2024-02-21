use candid::Principal;

use super::configuration::Configuration;

pub struct Inspect;

impl Inspect {
    /// Inspect if the caller is the archive canister
    pub fn inspect_is_archive_canister(caller: Principal) -> bool {
        Configuration::get_archive_canister() == caller
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr as _;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_inspect_is_archive_canister() {
        let principal = Principal::from_str("fgzua-6iaaa-aaaaq-aacgq-cai").unwrap();

        Configuration::set_archive_canister(principal);

        assert_eq!(
            Inspect::inspect_is_archive_canister(Principal::anonymous()),
            false
        );
        assert_eq!(Inspect::inspect_is_archive_canister(principal), true);
    }
}
