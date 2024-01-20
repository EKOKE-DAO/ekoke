use crate::utils::time;

/// Found value and its timestamp
pub struct IcCache<T>(T, u64);

impl<T> IcCache<T> {
    pub fn new(value: T, expires_at: u64) -> Self {
        Self(value, expires_at)
    }

    pub fn get(&self) -> Option<&T> {
        if self.is_expired(time()) {
            return None;
        }
        Some(&self.0)
    }

    pub fn is_expired(&self, now: u64) -> bool {
        self.1 < now
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_should_return_none_if_expired() {
        let cache = IcCache::new(1, 0);
        std::thread::sleep(Duration::from_millis(10));
        assert!(cache.get().is_none());
    }

    #[test]
    fn test_should_return_value_if_not_expired() {
        let cache = IcCache::new(1, time() + (86400 * 1_000_000_000));
        assert_eq!(cache.get(), Some(&1));
    }
}
