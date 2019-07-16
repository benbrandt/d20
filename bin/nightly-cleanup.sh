#!/bin/bash

# Cleanup old toolchains
rustup toolchain list | grep "nightly" | while read -r line
do
    # If not current specified toolchain, uninstall it
    if ! (echo $line | grep $(cat rust-toolchain) --quiet)
    then
        rustup toolchain uninstall $line
    fi
done
