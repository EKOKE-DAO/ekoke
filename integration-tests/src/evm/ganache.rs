use std::borrow::Cow;

use testcontainers::core::WaitFor;
use testcontainers::Image;

const NAME: &str = "trufflesuite/ganache";
const TAG: &str = "v7.9.2";

#[allow(missing_docs)]
// not having docs here is currently allowed to address the missing docs problem one place at a time. Helping us by documenting just one of these places helps other devs tremendously
#[derive(Debug, Default, Clone)]
pub struct Ganache {
    /// (remove if there is another variable)
    /// Field is included to prevent this struct to be a unit struct.
    /// This allows extending functionality (and thus further variables) without breaking changes
    _priv: (),
}

impl Image for Ganache {
    fn name(&self) -> &str {
        NAME
    }

    fn tag(&self) -> &str {
        TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout("RPC Listening on 0.0.0.0:8545")]
    }

    fn cmd(&self) -> impl IntoIterator<Item = impl Into<Cow<'_, str>>> {
        [
            "--chain.vmErrorsOnRPCResponse=true",
            "--wallet.totalAccounts=10",
            "--wallet.defaultBalance=1000",
            "-m 'candy maple cake sugar pudding cream honey rich smooth crumble sweet treat'",
        ]
    }
}
