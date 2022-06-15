# Substrate Cumulus Parachain Template

A new [Cumulus](https://github.com/paritytech/cumulus/)-based Substrate node, ready for hacking ☁️..

This project is originally a fork of the
[Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)
modified to include dependencies required for registering this node as a **parathread** or
**parachain** to a **relay chain**.

The stand-alone version of this template is hosted on the
[Substrate Devhub Parachain Template](https://github.com/substrate-developer-hub/substrate-parachain-template/)
for each release of Polkadot. It is generated directly to the upstream
[Parachain Template in Cumulus](https://github.com/paritytech/cumulus/tree/master/parachain-template)
at each release branch using the
[Substrate Template Generator](https://github.com/paritytech/substrate-template-generator/).

👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and
parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).


🧙 Learn about how to use this template and run your own parachain testnet for it in the
[Devhub Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/).

Run your Local Network
-----------

Launch a local setup including a Relay Chain and a Parachain.
Note: local PARA_ID is defaulted to 2000

### Launch the Relay Chain

```bash
# Compile Polkadot with the real overseer feature
git clone https://github.com/paritytech/polkadot
cargo build --release

# Alice
./target/release/polkadot \
--alice \
--validator \
--tmp \
--chain ../OAK-blockchain/resources/rococo-local.json \
--port 30333 \
--ws-port 9944

# Bob (In a separate terminal)
./target/release/polkadot \
--bob \
--validator \
--tmp \
--chain ../OAK-blockchain/resources/rococo-local.json \
--port 30334 \
--ws-port 9945
```

### Reserve the Parachain slot

1. Navigate to [Local relay parachain screen](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains/parathreads)
2. Click `+ ParaId`
3. Reserve `paraid` with the following paramaters
    - `reserve from`: `Alice`
    - `parachain id`: 1999
    - `reserved deposit`: <whatever the default is>


### Prep the Parachain

```bash
# Compile
cargo build --release

# Export genesis state
./target/release/parachain-collator export-genesis-state > genesis-state

# Export genesis wasm
./target/release/parachain-collator export-genesis-wasm > genesis-wasm

# Collator1
./target/release/parachain-collator \
--alice \
--collator \
--force-authoring \
--tmp \
--port 50333 \
--ws-port 9947 \
-- \
--execution wasm \
--chain resources/rococo-local.json \
--port 30335 \
--ws-port 9977 
```

### Register the parachain

1. Navigate to [Local relay sudo extrinsic](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/sudo)
2. Register your local parachain with the local relay chain (see the image below for the extrinsic). 
Note** the files mentioned are the ones you generated above.

![image](https://user-images.githubusercontent.com/2915325/99548884-1be13580-2987-11eb-9a8b-20be658d34f9.png)


### Test the parachain

1. Navigate to [Local parachain](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9947#/explorer)
2. WAIT.... It takes a while for the registration process to finish. 