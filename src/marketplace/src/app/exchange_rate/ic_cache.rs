use crate::utils::time;

pub enum IcCache<T> {
    /// Found value and its timestamp
    Hit(T, u64),
    /// Value not found
    Miss,
}

impl<T> IcCache<T> {
    pub fn new(value: T, expires_at: u64) -> Self {
        Self::Hit(value, expires_at)
    }

    pub fn get(&self) -> Option<&T> {
        if self.is_expired(time()) {
            return None;
        }
        match self {
            Self::Hit(value, _) => Some(value),
            Self::Miss => None,
        }
    }

    pub fn is_expired(&self, now: u64) -> bool {
        match self {
            Self::Hit(_, expires_at) => *expires_at < now,
            Self::Miss => true,
        }
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
