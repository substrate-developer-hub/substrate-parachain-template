# Substrate Cumulus Parachain Template

A new Cumulus-based Substrate node, ready for hacking :cloud:

## Upstream

This project is a fork of the
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).

## Build & Run

Follow these steps to prepare a local Substrate development environment :hammer_and_wrench:

### Setup

If necessary, refer to the setup instructions at the
[Substrate Developer Hub](https://substrate.dev/docs/en/knowledgebase/getting-started/#manual-installation).

### Build

Once the development environment is set up, build the node template. This command will build the
[Wasm](https://substrate.dev/docs/en/knowledgebase/advanced/executor#wasm-execution) and
[native](https://substrate.dev/docs/en/knowledgebase/advanced/executor#native-execution) code:

```bash
cargo build --release
```
## Run

### Local Relay Chain Testnet

#### Relay Chain Network(Validators)

We need to clone and install the Polkadot (rococo-v1 branch):
```bash
# Get a fresh clone, or `cd` to where you have polkadot already:
git clone git@github.com:paritytech/polkadot.git
cd polkadot
git checkout rococo-v1

# build WITH the real-overseer (required) 
cargo build --release --features real-overseer

# generaete the chainspec - note this file MUST be shared with all nodes!
# Other nodes cannot generate it due to possible non-determanism 
./target/release/polkadot build-spec --chain rococo-local --raw --disable-default-bootnode > rococo_local.json

# Start Relay `Alice` node
./target/release/polkadot --chain ./rococo_local.json -d cumulus_relay0 --validator --alice --port 50556
```

Open a new terminal, same directory: 

```bash 
# Start Relay `Bob` node
./target/release/polkadot --chain ./rococo_local.json -d cumulus_relay1 --validator --bob --port 50555
```

> There _must_ be a minimum of 2 relay chain nodes per parachain node. Scale as needed!

#### Parachain Nodes (Collators)

Substrate Parachain Template:
```bash
# NOTE: this command assumes the chain spec is in a directory named polkadot that is a sibling of the working directory
./target/release/parachain-collator -d local-test --collator --alice --ws-port 9945 --parachain-id 200 -- --chain ../polkadot/rococo_local.json
```

> Note: this chainspec file MUST be shared with all nodes genereated by _one_ validator node and passed around.
> Other nodes cannot generate it due to possible non-determanism 

### Registering on Local Relay Chain

#### Export the Parachain Genesis and Runtime

The files you will need to register we will generate in a `./resources` folder, to build them because
you modified the code you can use the following commands:

```bash
# Build the parachain node (from it's top level dir)
cargo build --release

# Build the Chain spec
./target/release/parachain-collator build-spec \
--disable-default-bootnode > ./resources/template-local-plain.json

# Build the raw file
./target/release/parachain-collator build-spec \
--chain=./resources/template-local-plain.json \
--raw --disable-default-bootnode > ./resources/template-local.json


# Export genesis state to `./resources files
./target/release/parachain-collator export-genesis-state --parachain-id 200 > ./resources/para-200-genesis
# export runtime wasm
./target/release/parachain-collator export-genesis-wasm > ./resources/para-200.wasm
```

#### Register on the Relay with `sudo`

In order to produce blocks you will need to register the parachain as detailed in the [Substrate Cumulus Worship](https://substrate.dev/cumulus-workshop/#/en/3-parachains/2-register) by going to 

`Developer -> sudo -> paraSudoWrapper -> sudoScheduleParaInitialize(id, genesis)`

Ensure you set the `ParaId to 200` and the `parachain: Bool to Yes`.

The files you will need are in the `./resources` folder, you just created.

## Learn More

Refer to the upstream
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
to learn more about the structure of this project, the capabilities it encapsulates and the way in
which those capabilities are implemented. You can learn more about
[The Path of Parachain Block](https://polkadot.network/the-path-of-a-parachain-block/) on the
official Polkadot Blog.
