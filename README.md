# Blockchain Engineering Interview Project

Welcome to the Blockchain Engineering Interview Project!

The goal of this project is to design and implement a service for tracking account balances in a blockchain.

Read over and follow the steps below.

## Step 1

- Read the [Background](#background) section below.
- Read `tests.rs` and `blocks.rs` to understand the test cases.
- Complete the expected balances in `tests.rs`.
- Push your changes to a new branch, create a pull request and share the link with us over Discord.

## Step 2

- Read `lib.rs` to understand the data structures and the `Service` trait. Note the sections marked with `TODO`.
- Implement the `Service` trait in `lib.rs`.
- Prioritize passing the tests and only then focus on performance.
- Ensure all tests pass by running `cargo test`.

## Step 3

- Write your system design in `DESIGN.md`.

## Step 4

- Read `example.rs` to understand how to use the Lightning Memory-mapped Database (LMDB) and serde libraries.
- Implement persistence in `lib.rs` so that the service can be restarted without losing data.
- Add an integration test in `tests.rs` that restarts the service and verifies that the account balances are correct.

Good luck!

## Background

A **blockchain** is a tree of blocks.

A **block** is a bundle of transactions with an optional parent block.

A **transaction** represents a change in account balances.

Here is the example blockchain we'll use:

         A
        / \
       B   C
           |
           D

- Block A is the genesis block which has no parent block.
- Block B descends from block A.
- Block C descends from block A.
- Block D descends from block C.

The **genesis block** is the block with no parent block. In this example, block A is the genesis block.

A **leaf block** is a block with no children. In this example, blocks B and D are leaf blocks.

A **chain** or **fork**  is a list of blocks related by parent-child relationships starting from the genesis block and ending with a leaf block. In the example, chains are [A, B] and [A, C, D].

The **canonical chain** is the longest chain of the blockchain tree.
In other words, the canonical chain is the chain of blocks with the most blocks.
In this example, the canonical chain is A -> C -> D.

The **canonical chain tip** is the last block in the canonical chain. In this example, the canonical chain tip is D.

Each block contains a list of transactions. For example:

- Block A:
  - Mint 10 coins to Alice
  - Transfer 5 coins from Alice to Bob
- Block B:
  - Transfer 5 coins from Bob to Alice
- Block C:
  - Transfer 3 coins from Bob to Alice
- Block D:
  - Transfer 2 coins from Alice to Bob

We can compute the account balances for each account at each block by following the chain of blocks from the genesis block to the current block. For example, after ingesting blocks A and B only, the account balances for the canonical tip would be:
  
- Prior to ingesting any blocks:
  - Balances: Alice: 0, Bob: 0
- After ingesting Block A:
  - Balances: Alice: 5, Bob: 5
- After ingesting Block B:
  - Balances: Alice: 10, Bob: 0

Here are some design considerations:

- What should be the canonical chain if there were more than one chain of the same length?
- How should you handle receiving blocks out-of-order?
