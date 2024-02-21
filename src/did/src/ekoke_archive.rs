use candid::{self, CandidType, Deserialize, Int, Nat, Principal};

pub use crate::ekoke_index::Transaction;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EkokeArchiveInitData {
    /// ID of the canister that we need to forward transactions to
    pub index_id: Principal,
    /// ID of ekoke-ledger canister
    pub ledger_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksArg {
    pub start: Nat,
    pub length: Nat,
}

pub type Map = Vec<(String, Box<Value>)>;
#[derive(CandidType, Deserialize)]
pub enum Value {
    Int(Int),
    Map(Map),
    Nat(Nat),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Vec<Box<Value>>),
}

pub type Block = Box<Value>;
#[derive(CandidType, Deserialize)]
pub struct GetBlocksRet {
    pub blocks: Vec<Block>,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsArg {
    pub start: Nat,
    pub length: Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsRet {
    pub transactions: Vec<Transaction>,
}
