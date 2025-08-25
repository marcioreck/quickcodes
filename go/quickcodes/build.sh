#!/bin/bash

# Build the Rust library with all features
cd ../..
cargo build --release --features "readers png svg pdf"

# Run Go tests with local library path
cd go/quickcodes
export LD_LIBRARY_PATH="../../target/release:$LD_LIBRARY_PATH"
go test -v