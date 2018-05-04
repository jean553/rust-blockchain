# rust-blockchain

A very simple blockchain in Rust.

## In progress

 * multiple-nodes (synchronize chain from other nodes chains)

## Table of content

 * [Dependencies](#dependencies)
 * [Block structure](#block-structure)
 * [Start nodes](#start-nodes)
    - [Create many nodes](#create-many-nodes)
    - [Start a node](#start-a-node)
    - [Check node IP address](#check-node-ip-address)
 * [Find peers](#find-peers)
    - [Manually add a peer](#manually-add-a-peer)
    - [List peers](#list-peers)
 * [Blocks](#blocks)
    - [Add block](#add-block)
    - [List blocks](#list-blocks)

## Dependencies

 * `time`: time routines, used to easily get the current UTC timestamp (for block creation),
 * `sha1`: sha1 hashing routines, in order to hash the previous block of the current created one,
 * `bincode`: serialization routines, used to serialize a block into raw binary (used for hashing),
 * `serde/serde_derive`: serialization/deserialization routines, in order to make a block "serializable"
 * `termion`: provides terminal graphical routines

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

Every block of the chain contains the hash of the previous block.
This is exactly how "blockchain" maintains integrity
(for instance, changing a block data from the middle of the chain
would require to change all the blocks that come after).

## Start nodes

This section explains how to use the blockchain.

### Create many nodes

Blockchain usually works with multiple nodes. In order to create multiple nodes,
you can simply `git clone` or copy the project at different location,
rename the `PROJECT` environment variable from the Vagrantfile
(for instance, `rust_blockchain_0`, `rust_blockchain_1`, `rust_blockchain_2`...),
and execute `vagrant up` for each of these program.

(beware, do not copy the hidden directory `.vagrant/`)

### Start a node

To start a node, simply build the container:

```sh
vagrant up
```

Connect using SSH to the container:

```sh
vagrant ssh
```

Start the service:

```sh
./target/release/rust-blockchain
```

### Check node IP address

IP address is necessary when synchronizing the local blockchain of a node.
In order to check the private IP address of a node within the Docker LAN,
simply execute:

```sh
docker inspect rust_blockchain_dev
```

This is also possible to set the IP address of a node into the Vagrantfile:

```ruby
config.vm.network "private_network", ip: "10.10.10.10"
```

## Find peers

This section is about communication between nodes.

### Manually add a peer

The easiest way to link nodes together is to manually register a peer locally.
Simply add a peer with `add_peer`.

```sh
add_peer 172.0.0.10
```

### List peers

```sh
list_peers
```

## Blocks

This section is about chain blocks.

### Add block

Add a block and broadcast it to every peer.

For example, to add a block with the data `20`:

```sh
add_block 20
```

### List blocks

List all the blocks of the local chain:

```sh
list_blocks
```
