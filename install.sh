#!/bin/bash

cargo build --release

# Get the current working directory
current_dir=$(pwd)

# Define the path to the target
target="$current_dir/target/release/rsm_front"

# Define the symlink path
symlink_path="/usr/local/bin/rsm"

# Remove existing symlink if it exists
if [ -L "$symlink_path" ]; then
	echo "WARNING: symlink to rsm already exists, removing it"
	sudo rm "$symlink_path"
fi

# Create the symlink
sudo ln -s "$target" "$symlink_path"
