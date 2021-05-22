# Substrate Cumulus Parachain Template

A new [Cumulus](https://github.com/paritytech/cumulus/)-based Substrate node, ready for hacking :cloud:

This project is a fork of the
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
modified to include dependencies required for registering this node as a **parathread or parachian**
to an established **relay chain** 

## Build & Run

Follow these steps to prepare a local Substrate development environment :hammer_and_wrench:

### Setup of Machine

If necessary, refer to the setup instructions at the
[Substrate Developer Hub](https://substrate.dev/docs/en/knowledgebase/getting-started/#manual-installation).

### Build

Once the development environment is set up, build the parachain node template. This command will
build the
[Wasm Runtime](https://substrate.dev/docs/en/knowledgebase/advanced/executor#wasm-execution) and
[native node](https://substrate.dev/docs/en/knowledgebase/advanced/executor#native-execution) code:

```bash
cargo build --release
```

## Run A Collator Node

### Rococo Relay Chain Testnet

--- 
### _IS THIS TEMPLATE ROCOCO COMPATIBLE?_
> :white_check_mark: **Yes!** :white_check_mark:
>
> As of 5/5/2021 
---

Rococo is Parity's official relay chain testnet for connecting cumulus-based parathreads
and parachains.

**See the [Cumulus Workshop](https://substrate.dev/cumulus-workshop/) for the latest instructions**
**to register a parathread/parachain on Rococo**

> **IMPORTANT NOTE:** you _must_ use the _same_ commit for cumulus and polkadot `rococo-v1` branch
> to build your parachain against to be compatible!!! You _must_ test locally registering your
> parachain successfully before you attempt to connect to rococo!

- **[Polkadot `rococo-v1` branch](https://github.com/paritytech/polkadot/tree/rococo-v1)**
- **[Cumulus `rococo-v1` branch](https://github.com/paritytech/cumulus/tree/rococo-v1)**

This network is under _constant development_ - so expect to need to follow progress and update
your parachains in lock step with the rococo changes if you wish to connect to the network.

Do join the [rococo matrix chat room](https://matrix.to/#/#rococo:matrix.parity.io) to ask
questions and connect with the rococo teams.

### Local Relay Chain Testnet

To operate a parathread or parachain, you _must_ connect to a relay chain.

#### Relay Chain Network (Validators)

Clone and build the Polkadot (**`rococo-v1` branch**):
```bash
# Get a fresh clone, or `cd` to where you have polkadot already:
git clone -b rococo-v1 --depth 1 https://github.com:paritytech/polkadot.git
cd polkadot
cargo build --release
```

##### Generaete the chainspec

> NOTE: this file _must_ be generated on a _single node_ and then the file shared with all nodes!
> Other nodes _cannot_ generate it due to possible non-determinism. 

```bash
./target/release/polkadot build-spec \
--chain rococo-local \
--raw \
--disable-default-bootnode \
> rococo_local.json
```

##### Start Relay Chain Node(s)

You should have a minimum of 2 running full _validator_ nodes on your relay chain per parachain/thread
collator you intend to connect!

From the Polkadot working directory:
```bash
# Start Relay `Alice` node
./target/release/polkadot\
--chain ./rococo_local.json \
-d cumulus_relay/alice \
--validator \
--alice \
--port 50555
```

Open a new terminal, same directory: 

```bash
# Start Relay `Alice` node
./target/release/polkadot\
--chain ./rococo_local.json \
-d cumulus_relay/bob \
--validator \
--bob \
--port 50556
```
Add more nodes as needed, with non-conflicting ports, DB directiories, and validator keys
(`--charlie`, `--dave`, etc.).

#### Parachain Nodes (Collators)

From the parachain template working directory:

```bash
# NOTE: this command assumes the chain spec is in a directory named `polkadot`
# that is at the same level of the template working directory. Change as needed.
./target/release/parachain-collator \
-d cumulus-parachain/alice \
--collator \
--alice \
--force-authoring \
--ws-port 9945 \
--parachain-id 200 \
-- \
--execution wasm \
--chain ../polkadot/rococo_local.json
```

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
./target/release/parachain-collator export-genesis-wasm > ./resources/para-200-wasm
```

> Note: we have set the `para_ID = 200` here, this _must_ be unique for all parathreads/chains on the
> relay chain you register with.

#### Register on the Relay with `sudo`

In order to produce blocks you will need to register the parachain as detailed in the
[Substrate Cumulus Worship](https://substrate.dev/cumulus-workshop/#/en/3-parachains/2-register)
by going to:

`Developer -> sudo -> paraSudoWrapper -> sudoScheduleParaInitialize(id, genesis)`

Ensure you set the `ParaId to 200` and the `parachain: Bool to Yes`.

The files you will need are in the `./resources` folder, you just created.

> Note : When registering to the public Rococo testnet, ensure you set a **unique** 
> `para_id` > 1000, below 1000 is reserved _exclusively_ for system parachains.

#### Restart the Parachain (Collator) and Wait...

The collator node may need to be restarted to get it functioning as expected. After a 
[new era](https://wiki.polkadot.network/docs/en/glossary#era) starts on the relay chain,
your parachain will come online. Once this happens, you should see the collator start
reporting _parachian_ blocks:

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

- More detailed instructions to use Cumulus parachains are found in the
[Cumulus Worship](https://substrate.dev/cumulus-workshop/#/en/3-parachains/2-register)
- Refer to the upstream
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
to learn more about the structure of this project, the capabilities it encapsulates and the way in
which those capabilities are implemented.
- You can learn more about
[The Path of Parachain Block](https://polkadot.network/the-path-of-a-parachain-block/) on the
official Polkadot Blog.
