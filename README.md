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

## Connect a Collator Node to a Relay Chain 

### Local Relay Chain Testnet

To operate a parathread or parachain, you _must_ connect to a relay chain. You have a few choices,
the most typical for testing is a local development Rococo network, then moving to the live testnet.
**Keep in mind, you need to configure the specific relay chain you will connect to in your**
**collator `chain_spec.rs`**. In the following examples, we will use a `rococo-local` relay network.

#### Relay Chain Network (Validators)

Clone and build Polkadot (**at the correct commit for your relay chain**):
```bash
# Get a fresh clone, or `cd` to where you have polkadot already:
git clone -b <YOUR RELAY CHAIN BRANCH OR RELEASE TAG> --depth 1 https://github.com:paritytech/polkadot.git
cd polkadot
cargo build --release
```

##### Generate the chainspec

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
./target/release/polkadot \
--chain ./rococo_local.json \
-d /tmp/relay/alice \
--validator \
--alice \
--port 50555
```

Open a new terminal, same directory: 

```bash
# Start Relay `Bob` node
./target/release/polkadot \
--chain ./rococo_local.json \
-d /tmp/relay/bob \
--validator \
--bob \
--port 50556
```
Add more nodes as needed, with non-conflicting ports, DB directories, and validator keys
(`--charlie`, `--dave`, etc.).

##### Reserve a ParaID

To connect to a relay chain, you must first _reserve a `ParaId` for your parathread that will 
become a parachain. To do this, you _must_ have currency available on an account on that network
in sufficient amount to reserve an ID. This is 20 "units" on the testnets, check for the amount
on your relay chain. The relay chain will increment starting at `2000` for all chains connecting
that are not "systems parachains" that use a different method to obtain a `ParaId`.

The easiest way to reserve your `ParaId` this is via the
[Polkadot Apps UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains/parathreads)
under the Parachains -> Parathreads tab and use the `+ ParaID` button.

> You will need to connect to a _relay chain node_ to submit this extrinsic!
> In testnets, your ParaId will be 2000 for your first parathread registration.

In this example flow, we will use the **`Charlie` development account** where we have funds available. 
Once you submit this extrinsic successfully, you can start your collators.

### Parachain Network

#### Select the Correct Relay Chain

To operate your parachain, it _must_ connect to the _correct_ relay chain. 
**Keep in mind, you need to configure the specific relay chain you will connect to in your**
**collator `chain_spec.rs`**. Specifically you pass the command for the network you need in
the `Extensions` section of your `ChainSpec::from_genesis(` section:

```rust
    Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: id.into(),
		},
```

> You can choose from any pre-set runtime chain spec in the Polkadot repo looking in the 
> `cli/src/command.rs` and `node/service/src/chain_spec.rs` files or generate your own and use
> that. See the [Cumulus Workshop](https://substrate.dev/cumulus-workshop/) for how.

In the following examples, we use the `rococo-local` relay network we setup in the last section.

#### Export the Parachain Genesis and Runtime

The files you will need to register we will generate in a `./resources` folder, to build them because
you modified the code you can use the following commands:

```bash
# Build the parachain node (from it's top level dir)
cargo build --release

# Place to store files we need
mkdir -p resources 

# Build the Chain spec
./target/release/parachain-collator build-spec \
--disable-default-bootnode > ./resources/template-local-plain.json

# Build the raw file
./target/release/parachain-collator build-spec \
--chain=./resources/template-local-plain.json \
--raw --disable-default-bootnode > ./resources/template-local.json

# Export genesis state to `./resources files
# Assumes ParaId = 2000 . Change as needed.
./target/release/parachain-collator export-genesis-state --parachain-id 2000 > ./resources/para-2000-genesis
# export runtime wasm
./target/release/parachain-collator export-genesis-wasm > ./resources/para-2000-wasm
```

> Note: we have set the `para_ID = 2000` here, this _must_ be unique for all parathreads/chains on the
> relay chain you register with. You _must_ reserve this first on the relay chain!

#### Start Parachain Nodes (Collators)

From the parachain template working directory:

```bash
# NOTE: this command assumes the chain spec is in a directory named `polkadot`
# that is at the same level of the template working directory. Change as needed.
#
# It also assumes a ParaId oof 2000. Change as needed.
./target/release/parachain-collator \
-d /tmp/parachain/alice \
--collator \
--alice \
--force-authoring \
--ws-port 9945 \
--parachain-id 2000 \
-- \
--execution wasm \
--chain ../polkadot/rococo_local.json
```

#### Register on the Relay with `sudo`

In order to produce blocks you will need to register the parachain as detailed in the
[Substrate Cumulus Worship](https://substrate.dev/cumulus-workshop/#/en/3-parachains/2-register)
by going to:

`Developer -> sudo -> paraSudoWrapper -> sudoScheduleParaInitialize(id, genesis)`

Ensure you set the `ParaId to 2000` and the `parachain: Bool to Yes`.

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

### Rococo & Westend Testnet Relay Chains

---

> _IS THIS TEMPLATE ROCOCO & WESTEND COMPATIBLE?_
> 
> **Yes!**
> 
> As of 30/05/2021 for Polkadot runtimes v0.9.3

---

**See the [Cumulus Workshop](https://substrate.dev/cumulus-workshop/) for the latest instructions**
**to register a parathread/parachain on a relay chain**

> **IMPORTANT NOTE:** you _must_ use the _same_ commit for cumulus and polkadot the present runtime
> for the network you are connecting to uses to be compatible!!!
> You _must_ test locally registering your parachain successfully before you attempt to connect to
> any running relay chain network!

Find `Chain spec` files to connect to live networks [here](https://github.com/paritytech/polkadot/tree/master/node/service/res).
You want to be sure to use the correct git release tag in these files, as they change from time
to time and _must_ match the live network! 

- **Rococo** is generally more unstable getting tests incorporated first, and reset often!
  - Join in the [Rococo Faucet](https://matrix.to/#/#rococo-faucet:matrix.org) to get some funds.
- **Westend** is more stable, and is not reset except when absolutely needed.
  - Join in the [Westend Faucet](https://matrix.to/#/#westend_faucet:matrix.org) to get some funds.

These networks are under _constant development_ - so expect to need to follow progress and update
your parachains in lock step with the testnet changes if you wish to connect to the network.

Do join the [rococo matrix chat room](https://matrix.to/#/#rococo:matrix.parity.io) to ask
questions and connect with the parachain building teams.

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
