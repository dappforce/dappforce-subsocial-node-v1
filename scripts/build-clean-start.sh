#!/usr/bin/env bash

cargo build --release
cargo run -- purge-chain --dev
cargo run -- --dev
