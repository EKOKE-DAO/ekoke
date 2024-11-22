pub use inner::*;

mod inner {
    use ethers_contract::abigen;

    abigen!(Deferred, "src/abi/deferred.json");
    abigen!(RewardPool, "src/abi/rewardPool.json");
}
