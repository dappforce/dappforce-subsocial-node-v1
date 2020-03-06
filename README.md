# Subsocial Full Node by [DappForce](https://github.com/dappforce)

Subsocial is a set of Substrate pallets with web UI that allows anyone to launch their own decentralized censorship-resistant social network aka community. Every community can be a separate Substrate chain and connect with other communities via a Polkadot-based relay chain.

You can think of this as decentralized versions of Reddit, Stack Exchange or Medium, where subreddits or communities of Stack Exchange or blogs on Medium run on their own chain. At the same time, users of these decentralized communities should be able to share their reputation or transfer coins and other values from one community to another via Polkadot relay chain.

To learn more about Subsocial, please visit [Subsocial Network](http://subsocial.network).

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

## Building from Docker

### Easiest start
To start Subsocial Full Node separately (you should have docker-compose):

```
cd docker/
docker-compose up -d
```

### Start with your own  parameters

```
docker run -p 9944:9944 dappforce/subsocial-node:latest ./subsocial-node [flags] [options]
```
* Don't forget `--ws-external` flag, if you want your node to be visible no only within the container.

### Build your own image
If you want to build docker image from your local repository (it takes a while...), in your shell:

```
cd docker/
./build
```

### Start all parts of Subsocial at once with [Subsocial-Starter](https://github.com/dappforce/dappforce-subsocial-starter)

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
