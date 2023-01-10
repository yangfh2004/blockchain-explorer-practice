#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod blocks;
mod tests;

type TransactionID = String;
type BlockID = String;

use std::collections::HashMap;

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

#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub balance: u64,
}

#[derive(Default)]
pub struct ServiceImpl {
    pub states: Vec<HashMap<String, Account>>,
    pub chains: Vec<Vec<Block>>,
    pub leaf_blocks: HashMap<BlockID, usize>,
}

impl Service for ServiceImpl {
    type Balance = u64;

    fn new() -> Self {
        Self {
            states: Vec::new(),
            chains: Vec::new(),
            leaf_blocks: HashMap::new(),
        }
    }

    fn ingest_block(&mut self, _block: &Block) -> anyhow::Result<()> {
        if let Some(parent) = _block.parent_id.clone() {
            println!("parent id {}", parent);
            if let Some(idx) = self.leaf_blocks.get(&parent) {
                self.chains[*idx].push(_block.clone());
                self.leaf_blocks.insert(_block.block_id.clone(), idx.clone());
                self.leaf_blocks.remove(&parent);
                for tx in &_block.transactions {
                    match tx {
                        Transaction::Mint { tx_id, to, amount } => {
                            let mut last_state = self.states.last_mut().unwrap();
                            let mut account = last_state.get_mut(to).unwrap();
                            account.balance += amount;
                        },
                        Transaction::Transfer { tx_id, from, to, amount } => {
                            let balance = self.get_balance(from)?;
                            if *amount <= balance {
                                let mut last_state = self.states.last_mut().unwrap();
                                let mut to_account = last_state.get_mut(to).unwrap();
                                to_account.balance += amount;
                                let mut from_account = last_state.get_mut(from).unwrap();
                                from_account.balance -= amount;
                            }
                        }
                    }
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
                            for block in &chain[..(i+1)] {
                                fork.push(block.clone());
                            }
                            fork.push(_block.clone());
                            // rewind all txs
                            let mut new_state: HashMap<String, Account> = HashMap::new();
                            for block in fork.clone() {
                                for tx in &block.transactions {
                                    match tx {
                                        Transaction::Mint { tx_id, to, amount } => {
                                            if let Some(to_account) = new_state.get_mut(to) {
                                                to_account.balance += amount;
                                            } else {
                                                let mut account = Account {
                                                    id: to.clone(),
                                                    balance: amount.clone(),
                                                };
                                                new_state.insert(to.clone(), account);
                                            }

                                        },
                                        Transaction::Transfer { tx_id, from, to, amount } => {
                                            if let Some(from_account) = new_state.get_mut(from) {
                                                if from_account.balance >= *amount {
                                                    from_account.balance -= amount;
                                                }
                                            }
                                            if let Some(to_account) = new_state.get_mut(to) {
                                                to_account.balance += amount;
                                            } else {
                                                let mut to_account = Account {
                                                    id: to.clone(),
                                                    balance: amount.clone(),
                                                };
                                                new_state.insert(to.clone(), to_account);
                                            }
                                        }
                                    }
                                }
                            }
                            self.states.push(new_state);
                            self.chains.push(fork);
                            self.leaf_blocks.insert(_block.block_id.clone(), self.chains.len()-1);
                        }
                    }
                }
            }
        } else {
            let mut new_chain = Vec::new();
            new_chain.push(_block.clone());
            self.chains.push(new_chain);
            self.leaf_blocks.insert(_block.block_id.clone(), self.chains.len()-1);
            let mut new_state: HashMap<String, Account> = HashMap::new();
            for tx in &_block.transactions {
                match tx {
                    Transaction::Mint { tx_id, to, amount } => {
                        if let Some(to_account) = new_state.get_mut(to) {
                            to_account.balance += amount;
                        } else {
                            let mut account = Account {
                                id: to.clone(),
                                balance: amount.clone(),
                            };
                            new_state.insert(to.clone(), account);
                        }

                    },
                    Transaction::Transfer { tx_id, from, to, amount } => {
                        if let Some(from_account) = new_state.get_mut(from) {
                            if from_account.balance >= *amount {
                                from_account.balance -= amount;
                            }
                        }
                        if let Some(to_account) = new_state.get_mut(to) {
                            to_account.balance += amount;
                        } else {
                            let mut to_account = Account {
                                id: to.clone(),
                                balance: amount.clone(),
                            };
                            new_state.insert(to.clone(), to_account);
                        }
                    }
                }
            }
            self.states.push(new_state);
        }
        anyhow::Ok(())
    }

    fn get_balance(&self, _account: &str) -> anyhow::Result<Self::Balance> {
        let last_state = match self.states.last() {
            Some(state) => state,
            None => return anyhow::Ok(0),
        };
        let account = last_state.get(_account).unwrap();
        anyhow::Ok(account.balance.clone())
    }
}
