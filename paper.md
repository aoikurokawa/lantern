# Proof of Proof of Stake

## Abstract
- Our proofs of proof-of-stake(PoPos) take the form of a Merkle tree of PoS epochs.
- The verifier enrolls the provers in a bisection game, in which honest provers are destined to win once an adversarial Merkle tree is challenged at sufficient depth.
- Promises both efficiency and decentralization by achieving logarithmic bootstrapping complexity. This is achieved through a novel construction using a Merkle tree of PoS epochs and engaging provers in a bisection game, where honest provers can prevail against adversarial attempts at deception when the Merkle tree is sufficiently challenged.

## Introduction
- Efficiently verifying the proof-of-stake blockchain without downloading the whole PoS. -> proofs of proof of stake

## Construction Overview

### Full nodes
- A *full node* client boots holding only the genesis block and connects to other full nodes (known as provers) in order to synchronize to the latest tip. 
- The full node downloads the entire chain block-by-block verifying each transaction in the process.
- Once the client verifies the latest tip, it has calculated the latest state and can answer user queries such as "What is my balance?". 

### Sync committees
- PoS protocols typically proceed in epochs during which the validator set is fixed.
- In each epoch, a subset of validators is elected by the protocol as the epoch committee.
- The current committee signs the latest state. Therefore, the client does not need to download all the blocks, but instead only needs to download the latest state and verify the committee signatures on it.
- To perform the verification at some later epoch, the client needs to keep track of the current committee.

### Optimistic light client construction
- For each committee, take its members concatenate them all togerther, and hash them into one hash.
- The prover then sends this sequence of hashes (one for each committee) to the client. 

## Proof of Stake (PoS) superlight client

1. Merkle Tree of PoS Epochs
In a PoS blockchain, time is divided into epochs, which are periods during which a set of validators (provers) are responsible for validating transactions and creating new blocks. A Merkle tree is a cryptographic structure that efficiently summarizes all the data in a set, allowing for quick verification of whether a piece of data is part of the set. In this context, each node in the Merkle tree represents an epoch's state or summary. This allows the light client to quickly check the state of the blockchain at various points in time without needing to process every single block.

2. Bisection Game
This is a strategy to efficiently verify information. If a light client wants to verify the state of the blockchain at a specific epoch, it can challenge a prover (a full node) to provide proof. If the prover is dishonest, the light client can engage in a "bisection" process, where it asks the prover to provide proof for smaller and smaller intervals until it either finds a discrepancy or narrows down the interval to a point where it can directly verify the truth. Honest provers can quickly provide valid proofs, while dishonest ones are caught in their deception.


Papaer
- [Proofs of Proof-of-Stake with Sublinear Complexity](https://arxiv.org/pdf/2209.08673.pdf)