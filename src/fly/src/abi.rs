pub use inner::*;

mod inner {
    use ethers_contract::abigen;

    abigen!(Fly, "src/abi/fly.json");
}
