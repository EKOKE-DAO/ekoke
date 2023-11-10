use candid::{Nat, Principal};

/// Returns current time in nanoseconds
pub fn time() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        time.as_nanos() as u64
    }
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::time()
    }
}

pub fn cycles() -> Nat {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Nat::from(30_000_000_000_u64)
    }
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::canister_balance().into()
    }
}

pub fn caller() -> Principal {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
            .unwrap()
    }
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::caller()
    }
}

pub fn parse_date(date_str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    if date_str.len() != 10 {
        return Err("bad syntax".into());
    }
    let parts: Vec<&str> = date_str.split('-').collect();

    if parts.len() != 3 {
        return Err("bad syntax".into());
    }

    let year: i32 = parts[0].parse()?;
    let month: i32 = parts[1].parse()?;
    let day: i32 = parts[2].parse()?;

    let date = time::Tm {
        tm_year: year - 1900,
        tm_mon: month - 1,
        tm_mday: day,
        tm_hour: 0,
        tm_min: 0,
        tm_sec: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };

    Ok(date.to_timespec().sec as u64 * 1_000_000_000 + date.to_timespec().nsec as u64)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_parse_date() {
        assert_eq!(parse_date("2023-11-08").unwrap(), 1_699_401_600_000_000_000);
    }
}
