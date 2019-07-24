#!/usr/bin/env bash

set -e

echo "*** Initialising WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup update nightly
   rustup update stable
fi

rustup override set nightly-2019-07-14
rustup target add wasm32-unknown-unknown --toolchain nightly-2019-07-14

# Install wasm-gc. It's useful for stripping slimming down wasm binaries.
command -v wasm-gc || \
	cargo +nightly install --git https://github.com/alexcrichton/wasm-gc --force
