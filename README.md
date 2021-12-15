# Substrate Kitties

## Getting Started

Clone this repository. 

SSH: `git@github.com:hermanodecastro/kitties.git` 
or
HTTPS: `https://github.com/hermanodecastro/kitties.git`

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Build

To build this project run `cargo build --release`

### Run

Run `./target/release/node-kitties --tmp --dev` for a temporary chain in develop mode. You can also `run` a permissioned network starting the nodes. Go to `https://docs.substrate.io/tutorials/v3/permissioned-network/#alice-and-bob-start-the-network` for more informations.

### Frontend

Use `https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer` to interact with the network. 
More informations `https://docs.substrate.io/tutorials/v3/kitties/pt1/#testing-with-polkadot-js-apps-ui`.

# The project

The substrateKitties pallet is responsible for issuing and managing the kitties. While the substrateIdentity pallet is responsible for assigning an identity to a kitty.

## substrateKitties extrinsics 

```
- breedKitty(parent1, parent2)
- buyKitty(kittyId, bidPrice)
- createKitty()
- setPrice(kittyId, price)
- transfer(to, kittyId)
```

## substrateIndentity extrinsics

```
- assignIndetity(indentity, kittyId)
- getKittyByIdentity(identity)
```

### Example

```
- assignIdentity(hermano, 0xf4e2b03f7697096f7c78ff8642adfbafdeadf45df99f1db7f9ba77bcf2c69ed6) 
- returns (hashOf(hermano), kittyId)

- getKittyByIdentity(hermano) 
- returns 0xf4e2b03f7697096f7c78ff8642adfbafdeadf45df99f1db7f9ba77bcf2c69ed6 which is the kittyId.
```


