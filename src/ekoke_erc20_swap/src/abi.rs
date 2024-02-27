pub use inner::*;

mod inner {
    use ethers_contract::abigen;

    abigen!(Ekoke, "src/abi/ekoke.json");
}
