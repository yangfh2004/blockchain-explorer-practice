#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod blocks;
mod tests;

type TransactionID = String;
type BlockID = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Transaction {
    Mint {
        tx_id: TransactionID,
        to: String,
        amount: u64,
    },
    Transfer {
        tx_id: TransactionID,
        from: String,
        to: String,
        amount: u64,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {
    pub block_id: BlockID,
    pub parent_id: Option<BlockID>,
    pub transactions: Vec<Transaction>,
}

pub trait Service {
    type Balance: PartialEq + Eq + std::fmt::Debug;

    fn new() -> Self;
    fn ingest_block(&mut self, block: &Block) -> anyhow::Result<()>;
    fn get_balance(&self, account: &str) -> anyhow::Result<Self::Balance>;
}

#[derive(Default)]
pub struct ServiceImpl {}

impl Service for ServiceImpl {
    type Balance = String; //TODO: Decide how to represent balances

    fn new() -> Self {
        todo!()
    }

    fn ingest_block(&mut self, _block: &Block) -> anyhow::Result<()> {
        todo!()
    }

    fn get_balance(&self, _account: &str) -> anyhow::Result<Self::Balance> {
        todo!()
    }
}
