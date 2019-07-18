#!/bin/bash

# Installs all toolchain tools
rustup component add clippy
rustup component add rustfmt
# Make sure local dev environment will work as well for nightly toolchains
rustup component add rls
rustup component add rust-analysis
rustup component add rust-src
