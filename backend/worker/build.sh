#!/bin/bash
set -e

# Build the wasm module
cargo install worker-build --version 0.0.9
~/.cargo/bin/worker-build --release

# Copy the generated wasm to the build directory if it exists
if [ -f build/worker/geminipocket.wasm ]; then
    cp build/worker/geminipocket.wasm build/
fi