#!/bin/bash
set -e

cargo install worker-build --version 0.0.9
~/.cargo/bin/worker-build --release

if [ -f build/worker/geminipocket.wasm ]; then
    cp build/worker/geminipocket.wasm build/
fi