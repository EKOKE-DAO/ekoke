use candid::{Nat, Principal};
use time::{Date, OffsetDateTime};

/// Returns current time in nanoseconds
pub fn time() -> u64 {
    if cfg!(test) {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        return time.as_nanos() as u64;
    }

    ic_cdk::api::time()
}

pub fn cycles() -> Nat {
    if cfg!(test) {
        return Nat::from(30_000_000_000_u64);
    }

    ic_cdk::api::canister_balance().into()
}

pub fn caller() -> Principal {
    if cfg!(test) {
        return Principal::from_text(
            "zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae",
        )
        .unwrap();
    }

    ic_cdk::caller()
}

/// Returns current date
pub fn date() -> Date {
    let time = time();

    let date = OffsetDateTime::from_unix_timestamp_nanos(time as i128).unwrap();
    date.date()
}
