mod fly_client;

#[cfg(not(test))]
pub use fly_client::IcFlyClient;
pub use fly_client::{fly_client, FlyClient};
