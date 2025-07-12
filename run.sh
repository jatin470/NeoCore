#!/bin/bash

echo "🧠 Starting NeoCore..."

# Build the Rust project
cargo build --release

# Run the executable
./target/release/neocore
