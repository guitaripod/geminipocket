#!/bin/bash
cd worker && ~/.cargo/bin/worker-build --release && rm -rf ../build && mv build ../