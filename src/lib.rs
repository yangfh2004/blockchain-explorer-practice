#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod blocks;
mod tests;

type TransactionID = String;
type BlockID = String;

use std::collections::HashMap;
use std::path::Path;
use lmdb::{Database, DatabaseFlags, Environment, Transaction as DBTransaction, WriteFlags};
use tempdir::TempDir;

const DB_NAME: &str = "my_db";
const DIR_NAME: &str = "./test_database";
const DATA_KEY: &str = "data_key";

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
    fn from_db(dir: &str, db_filename: &str) -> Self;
    fn update_db(&mut self);
    fn ingest_block(&mut self, block: &Block) -> anyhow::Result<()>;
    fn get_balance(&self, account: &str) -> anyhow::Result<Self::Balance>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: String,
    pub balance: u64,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ServiceImpl {
    pub states: Vec<HashMap<String, Account>>,
    pub chains: Vec<Vec<Block>>,
    pub leaf_blocks: HashMap<BlockID, usize>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    db: Option<Database>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    env: Option<Environment>,
    pub path: Option<String>,
}

impl Service for ServiceImpl {
    type Balance = u64;

    fn new() -> Self {
        let tmp = TempDir::new(DIR_NAME).expect("failed to open tmpdir");
        let path = tmp.into_path();
        // println!("Path: {:}", &*path.to_str().unwrap());
        let mut builder = Environment::new();
        builder.set_max_dbs(16);

        let env = builder.open(&*path).expect("failed to open env");
        let db = env
            .create_db(Some(DB_NAME), DatabaseFlags::empty())
            .expect("failed to open db");
        let path_str = &*path.to_string_lossy();
        Self {
            states: Vec::new(),
            chains: Vec::new(),
            leaf_blocks: HashMap::new(),
            db: Some(db),
            env: Some(env),
            path: Some(path_str.to_string()),
        }
    }

    /// Deserialize the blockchain data from a database.
    fn from_db(dir: &str, db_filename: &str) -> Self {
        let path = Path::new(dir);

        let mut builder = Environment::new();
        builder.set_max_dbs(16);

        let env = builder.open(path).expect("failed to open env");
        let db = env
            .create_db(Some(db_filename), DatabaseFlags::empty())
            .expect("failed to open db");
        let rotxn = env.begin_ro_txn().expect("can't begin ro txn");
        let rbytes = rotxn.get(db, &DATA_KEY).expect("failed to get key");
        let rstr = std::str::from_utf8(rbytes).expect("failed to parse read bytes");
        let service: Self = serde_json::from_str(rstr).expect("failed to deserialize");
        Self {
            states: service.states,
            chains: service.chains,
            leaf_blocks: service.leaf_blocks,
            db: Some(db),
            env: Some(builder.open(path).expect("failed to open env")),
            path: service.path,
        }
    }

    /// Serialize and write all blockchains into the database.
    fn update_db(&mut self) {
        let wbytes = serde_json::to_string(self).expect("failed to serialize");
        if let Some(env) = &self.env {
            let mut rwtxn = env.begin_rw_txn().expect("can't begin rw txn");
            if let Some(db) = self.db {
                rwtxn
                    .put(db, &DATA_KEY, &wbytes, WriteFlags::empty())
                    .expect("put failed");
                rwtxn.commit().expect("commit failed for rwtxn");
            }
        }
    }

    fn ingest_block(&mut self, _block: &Block) -> anyhow::Result<()> {
        // Blockchain as a state machine, transition into a new state.
        fn state_transition(state: &mut HashMap<String, Account>, tx: &Transaction) {
            match tx {
                Transaction::Mint {
                    tx_id: _,
                    to,
                    amount,
                } => {
                    if let Some(to_account) = state.get_mut(to) {
                        to_account.balance += amount;
                    } else {
                        let account = Account {
                            id: to.clone(),
                            balance: *amount,
                        };
                        state.insert(to.clone(), account);
                    }
                }
                Transaction::Transfer {
                    tx_id: _,
                    from,
                    to,
                    amount,
                } => {
                    if let Some(from_account) = state.get_mut(from) {
                        if from_account.balance >= *amount {
                            from_account.balance -= amount;
                        }
                    }
                    if let Some(to_account) = state.get_mut(to) {
                        to_account.balance += amount;
                    } else {
                        let to_account = Account {
                            id: to.clone(),
                            balance: *amount,
                        };
                        state.insert(to.clone(), to_account);
                    }
                }
            }
        }

        if let Some(parent) = _block.parent_id.clone() {
            // println!("parent id {}", parent);
            if let Some(idx) = self.leaf_blocks.get(&parent) {
                self.chains[*idx].push(_block.clone());
                self.leaf_blocks.insert(_block.block_id.clone(), *idx);
                self.leaf_blocks.remove(&parent);
                let last_state = self.states.last_mut().unwrap();
                for tx in &_block.transactions {
                    state_transition(last_state, tx);
                }
            } else {
                // no leaf contains parent, search the previous blocks
                let current_chains = self.chains.clone();
                for chain in current_chains {
                    for (i, block) in chain.iter().enumerate() {
                        // discard orphaned blocks.
                        if parent == block.block_id {
                            // fork from here.
                            let mut fork = Vec::new();
                            for block in &chain[..(i + 1)] {
                                fork.push(block.clone());
                            }
                            fork.push(_block.clone());
                            // rewind all txs
                            let mut new_state: HashMap<String, Account> = HashMap::new();
                            for block in fork.clone() {
                                for tx in &block.transactions {
                                    state_transition(&mut new_state, tx);
                                }
                            }
                            self.states.push(new_state);
                            self.chains.push(fork);
                            self.leaf_blocks
                                .insert(_block.block_id.clone(), self.chains.len() - 1);
                        }
                    }
                }
            }
        } else {
            let new_chain = vec![_block.clone()];
            self.chains.push(new_chain);
            self.leaf_blocks
                .insert(_block.block_id.clone(), self.chains.len() - 1);
            let mut new_state: HashMap<String, Account> = HashMap::new();
            for tx in &_block.transactions {
                state_transition(&mut new_state, tx);
            }
            self.states.push(new_state);
        }
        self.update_db();
        anyhow::Ok(())
    }

    fn get_balance(&self, _account: &str) -> anyhow::Result<Self::Balance> {
        if self.states.is_empty() {
            return anyhow::Ok(0);
        }
        // find canonical chain
        let mut max_len = 0;
        let mut idx: usize = 0;
        for (i, state) in self.states.iter().enumerate() {
            if state.len() >= max_len {
                max_len = state.len();
                idx = i;
            }
        }
        let last_state = &self.states[idx];
        let account = last_state.get(_account).unwrap();
        anyhow::Ok(account.balance)
    }
}
