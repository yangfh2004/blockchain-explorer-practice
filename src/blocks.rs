use crate::{Block, Transaction};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALICE: String = "Alice".to_string();
    pub static ref BOB: String = "Bob".to_string();
    pub static ref BLOCK_A: Block = Block {
        block_id: "A".to_string(),
        parent_id: None,
        transactions: vec![
            Transaction::Mint {
                tx_id: "A1".to_string(),
                to: ALICE.to_string(),
                amount: 10,
            },
            Transaction::Transfer {
                tx_id: "A0".to_string(),
                from: ALICE.to_string(),
                to: BOB.to_string(),
                amount: 5,
            },
        ],
    };
    pub static ref BLOCK_E: Block = Block {
        block_id: "E".to_string(),
        parent_id: None,
        transactions: vec![
            Transaction::Mint {
                tx_id: "A1".to_string(),
                to: ALICE.to_string(),
                amount: 8,
            },
            Transaction::Transfer {
                tx_id: "A0".to_string(),
                from: ALICE.to_string(),
                to: BOB.to_string(),
                amount: 7,
            },
        ],
    };
    pub static ref BLOCK_B: Block = Block {
        block_id: "B".to_string(),
        parent_id: Some("A".to_string()),
        transactions: vec![Transaction::Transfer {
            tx_id: "B0".to_string(),
            from: BOB.to_string(),
            to: ALICE.to_string(),
            amount: 5,
        }],
    };
    pub static ref BLOCK_C: Block = Block {
        block_id: "C".to_string(),
        parent_id: Some("A".to_string()),
        transactions: vec![Transaction::Transfer {
            tx_id: "C0".to_string(),
            from: BOB.to_string(),
            to: ALICE.to_string(),
            amount: 3,
        }],
    };
    pub static ref BLOCK_D: Block = Block {
        block_id: "D".to_string(),
        parent_id: Some("C".to_string()),
        transactions: vec![Transaction::Transfer {
            tx_id: "D0".to_string(),
            from: ALICE.to_string(),
            to: BOB.to_string(),
            amount: 2,
        }],
    };
}
