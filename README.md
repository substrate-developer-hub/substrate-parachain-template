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

#### Restart the Parachain (Collator) and Wait...

The collator node may need to be restarted to get it functioning as expected. After a [new era](https://wiki.polkadot.network/docs/en/glossary#era) starts on the relay chain, your parachain will come online. Once this happens, you should see the
collator start reporting _parachian_ blocks:

```bash
2021-04-01 16:31:06 [Relaychain] ✨ Imported #243 (0x46d8…f394)    
2021-04-01 16:31:06 [Relaychain] 👴 Applying authority set change scheduled at block #191    
2021-04-01 16:31:06 [Relaychain] 👴 Applying GRANDPA set change to new set [(Public(88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee (5FA9nQDV...)), 1), (Public(d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69 (5GoNkf6W...)), 1)]    
2021-04-01 16:31:06 [Relaychain] 👴 Imported justification for block #191 that triggers command Changing authorities, signaling voter.    
2021-04-01 16:31:06 [Parachain] Starting collation. relay_parent=0x46d87d4b55ffcd2d2dde3ee2459524c41da48ac970fb1448feaa26777b14f394 at=0x85c655663ad333b1508d0e4a373e86c08eb5b5353a3eef532a572af6395c45be
2021-04-01 16:31:06 [Parachain] 🙌 Starting consensus session on top of parent 0x85c655663ad333b1508d0e4a373e86c08eb5b5353a3eef532a572af6395c45be    
2021-04-01 16:31:06 [Parachain] 🎁 Prepared block for proposing at 91 [hash: 0x078560513ac1862fed0caf5726b7ca024c2af6a28861c6c69776b61fcf5d3e1f; parent_hash: 0x85c6…45be; extrinsics (2): [0x8909…1c6c, 0x12ac…5583]]    
2021-04-01 16:31:06 [Parachain] Produced proof-of-validity candidate. pov_hash=0x836cd0d72bf587343cdd5d4f8631ceb9b863faaa5e878498f833c7f656d05f71 block_hash=0x078560513ac1862fed0caf5726b7ca024c2af6a28861c6c69776b61fcf5d3e1f
2021-04-01 16:31:06 [Parachain] ✨ Imported #91 (0x0785…3e1f)    
2021-04-01 16:31:09 [Relaychain] 💤 Idle (2 peers), best: #243 (0x46d8…f394), finalized #192 (0x9fb4…4b28), ⬇ 1.0kiB/s ⬆ 3.2kiB/s    
2021-04-01 16:31:09 [Parachain] 💤 Idle (0 peers), best: #90 (0x85c6…45be), finalized #64 (0x10af…4ede), ⬇ 1.1kiB/s ⬆ 1.0kiB/s    
2021-04-01 16:31:12 [Relaychain] ✨ Imported #244 (0xe861…d99d)    
2021-04-01 16:31:14 [Relaychain] 💤 Idle (2 peers), best: #244 (0xe861…d99d), finalized #193 (0x9225…85f1), ⬇ 2.0kiB/s ⬆ 1.6kiB/s    
2021-04-01 16:31:14 [Parachain] 💤 Idle (0 peers), best: #90 (0x85c6…45be), finalized #65 (0xdd20…d44a), ⬇ 1.6kiB/s ⬆ 1.4kiB/s    
``` 

> Note the delay here! It may take some time for your relaychain to enter a new era. 

## Learn More

Refer to the upstream
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
to learn more about the structure of this project, the capabilities it encapsulates and the way in
which those capabilities are implemented. You can learn more about
[The Path of Parachain Block](https://polkadot.network/the-path-of-a-parachain-block/) on the
official Polkadot Blog.
