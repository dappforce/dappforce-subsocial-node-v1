# SubSocial Full Node by [DappForce](https://github.com/dappforce)

SubSocial is a set of Substrate runtime modules (SRML) with UI that would allow anyone to launch their own decentralized censorship-resistant social network aka community. We are planning to follow a topology of Polkadot Network where every community will be running on its own Substrate chain and all these decentralized communities will be connected to our own Polkadot relay. This social networking relay could be connected to the official Polkadot Network.

You can think of this as decentralized versions of Reddit, Stack Exchange or Medium, where subreddits or communities of Stack Exchange or blogs on Medium run on their own chain. At the same time, users of these decentralized communities should be able to transfer or share their reputation, coins and other values from one community to another via Polkadot relay.

## Building from source

### Initial setup
If you want to build from source you will need the Rust [toolchain](https://rustup.rs/), openssl and llvm/libclang.

```bash
git clone git@github.com:dappforce/dappforce-subsocial-node.git
```

Initialise the WASM build environment:

```bash
cd dappforce-subsocial-node/
./init-wasm.sh
```

### Building
Clone the SubSocial runtime into the dappforce-subsocial-runtime directory:

```bash
git clone git@github.com:dappforce/dappforce-subsocial-runtime.git
```

Build the WASM runtime library:
```bash
./build-runtime.sh
```

Build the node (native code):
```bash
cargo build --release
```

### Running a public node
Run the node and connect to the public testnet
```bash
cargo run --release
```

### Installing a release build
This will install the executable `subsocial-node` to your `~/.cargo/bin` folder, which you would normally have in your `$PATH` environment.

```bash
cargo install --path ./
```

Now you can run

```bash
subsocial-node
```

## Development

### Running a local development node

```bash
cargo run --release -- --dev
```

### Cleaning development chain data
When making changes to the runtime library remember to purge the chain after rebuilding the node to test the new runtime.

```bash
cargo run --release -- purge-chain --dev
```

### Substrate node template

The full node is built based on the substrate node [template](https://github.com/shawntabrizi/substrate-package/tree/master/substrate-node-template)