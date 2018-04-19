# rust-blockchain

A very simple blockchain in Rust.

## In progress

 * multiple-nodes (synchronize chain from other nodes chains)

## Table of content

 * Dependencies
 * Block structure

## Dependencies

 * `time`: time routines, used to easily get the current UTC timestamp (for block creation),
 * `sha1`: sha1 hashing routines, in order to hash the previous block of the current created one,
 * `bincode`: serialization routines, used to serialize a block into raw binary (used for hashing),
 * `serde/serde_derive`: serialization/deserialization routines, in order to make a block "serializable"

## Block structure

A block of the ledger contains the following fields:
 * the `timestamp` of the block creation,
 * the `data` of the block (signed integer in order to be similar to cryptocurrencies),
 * the hash digest of the `previous` ledger block (empty string if the current block is the genesis one)
 * the hash digest of the `current` ledger block (stored as a string, hexadecimal digest)

The `previous` and `current` hash digests are directly fields of the block.
The `timestamp` and the `data` fields are part of a hashable content structure,
itself a field of the block structure.

In fact, in order to keep things as simple as possible, only the `timestamp` and
the `data` are hashed into `previous` and `current`.

The `Block` structure implements the `Serialize` trait. In order to keep everything simple,
and in order to prevent custom serialization functions, the block structure only contains
primitive types (`i64`, `i32` and `String`) as they all already implement the trait.
