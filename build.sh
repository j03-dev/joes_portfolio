#!usr/bin/env bash

cargo install trunk

rustup toolchain install nightly
rustup override set nightly
rustup target add wasm32-unknown-unknown
