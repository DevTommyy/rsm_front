#!/bin/bash

cargo build --release

# Get the current working directory
current_dir=$(pwd)

# Define the path to the target
target="$current_dir/target/release/rsm_front"

# Create the symlink
sudo ln -s "$target" /usr/local/bin/rsm
