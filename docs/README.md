# docs

There's a design image in this directory that is out of date, but roughly speaking follows what will be implemented in this repo. For now that's basically not readable for any person other than me.

These are the components (probably Rust crates) that I intend on creating:

- [x] HTTP Server
- [ ] Transaction Validator
- [ ] Transaction Receiver/Emitter
- [ ] Transaction Pool
- [ ] World State
- [ ] Transaction Executor ("EVM")
- [ ] Block Finisher
- [ ] Miner
- [ ] Block Emitter
- [ ] Block Validator ("Block Listener")
- [ ] Forker ("Consensus Mediator")

## Execution flow

1. "HTTP Server" receives a transaction, then passes it to the "Transaction Validator".
2. "Transaction Validator" validates the transaction (signature, etc), then sends it to the "Transaction Receiver/Emitter".
3. "Transaction Receiver/Emitter" receives from both "Transaction Validator" and from other "node"s. It also emits those valid transactions to these "node"s.
4. "Transaction Pool" orders the transaction list based of `gasPrice` (may be miner specific in the future).
5. "Transaction Executor" handles one transaction at a time, and pulls from "Transaction Pool", which does a pop from it's list, sending the "priciest" transaction.
6. The "Transaction Executor" pulls the current "World State" and mutates it if the transaction succeeded. Either way, the transaction will go to the "Block Finisher".
7. The "Block Finisher" works like the "Transaction Pool" (in the sense of keeping a list of transactions), however it serves as a "grouper" to make a block limited by a transaction **count** ("block size").
8. Once the block size has reached the threshold, it's sent to the "Miner".
9. The "Miner" tries to mine for a valid block given the transaction list it has.
10. Once it succeeds doing the mining, it will send the valid block to the "Block Emitter".
11. "Block Emitter" then sends the valid block to the network of "node"s.
12. "Block Validator" listens for external blocks and validates that it is wellformed. If it is, it will send the block to the "Forker"
13. The "Forker" listens for externally mined blocks, and locks the "World State" if the local node forked for too long. (kinda complicated component, need to think more about this one)
