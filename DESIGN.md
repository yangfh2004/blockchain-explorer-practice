# System Design

Describe the design of your solution.

Feel free to use your own structure or template and be sure to address at the very least:

- What are the major components?

    The major components are in the ServiceImpl are states, chains and leaf blocks.


- How do they interact?
    
    The states represent a number of forked chains and their state machine. Each state contains a hash map of accounts. 
The chains are a list of forked blockchains. The leaf blocks are leaf blocks for each chain. If there is a new block 
coming in, the server will detect if it is a genesis block, if so, a new chain will be created from the genesis block.
Moreover, the code will check if it is a child of a leaf block, if not, the server will search from all earlier blocks 
to find its potential parents. If there is a parent in earlier block, the new block will create a fork. The server will create 
a newly forked chain. If a block is an orphaned block, which means it has no parent, it will be discarded.

    Whenever a new block is added to a blockchain, its corresponding state, which contains a number of accounts will be 
updated based the transaction. If the account sending tokens does not have enough balance, its transaction will be rejected 
and discarded.

    When querying the balance of an account, the service will uses the longest chain as a canonical chain. If there are two 
chains has the same length, the chain whose leaf node is newer will be used.


- How does your solution scale?

    The solution can use more efficient data structures for queries and mutations. Current implementation iterates all early 
blocks to find a parent of a new block. It may use a hashmap to store the index of the blockchain and its block id to locate a 
parent block. A binary heap can be used to find the longest chain. Eventually, all blockchain data can be stored in a secondary 
database and most recent blocks can be cached in the memory since recent data will be queried most.

    While a single server cannot serve all new requests, a fault-tolerated distributed network can be built on top of [RAFT algorithm](https://raft.github.io).
The blockchains are replicated state machines across a distributed system since a log of state transitions (Blocks) are replicated since only
the leader node append the new entries (new blocks) to the log. Since a blockchain explorer are mostly used to read the records of transactions 
on the blockchain, a number of distributed nodes can deal with a large number of queries concurrently. A load balancer will distribute 
HTTP requests to the backend nodes.


- What are some of the tradeoffs you made?

    I discard orphaned blocks since it would be a hassle to deal with them in this project and try to link them into the blockchain. If we have to
deal with them, I would like to keep them a pool and wait a number of new blocks. If the block still cannot find its parents after a 
number of new blocks, I would like to believe this block is truly an orphan and discard it forever.

    As mentioned in above answers, I use simplified data structure to implement the system quickly. Advanced data structure will improve 
the performance of this solution.


- What are some of the things you would do differently if you had more time?

    I will build a tree structures instead of copying some part of a blockchain while forking. It will definitely save more memory and 
serve other advanced search algorithms. I will implement every data structures mentioned above if I have more time. I will write benchmarking 
code and more tests for corner cases.


- How does your solution handle receiving blocks out-of-order?

    I mentioned in the above answers. I discard orphaned blocks. If I have more time, I will create a pool of orphaned blocks and try to 
add them into the blockchain every time a new block comes.