#!/bin/bash
# ----------------------------------------------------------------------------------------------------
# | This script creates the base for the log (if the app doesn't find the log file it's gonna break) |
# | builds the binary and puts the binary into a path where it can be ran from anywhere              |
# ----------------------------------------------------------------------------------------------------

# Find the directory containing cli_client and its subdir log
# NOTE: change this if the installer is failing to find the dir
# you can change where it searches by changing the dirs
cli_client_dir=$(find ~/Codes ~/Documents ~/Downloads ~/Desktop -type d -name "cli_client" 2>/dev/null)
log_dir="$cli_client_dir/log"

# Create the log directory if it doesn't exist
mkdir -p "$log_dir"

# Create the log file rsm-log.log inside log directory
log_file="$log_dir/rsm-log.log"
touch "$log_file"

# Run cargo build --release
cd "$cli_client_dir" || exit

# Create the file that contains the locations
env_dir="$cli_client_dir/.env"
touch "$env_dir"

# Store the env path for the bin
env_path="$cli_client_dir/src/env_path.txt"
touch "$env_path"
echo "$env_dir" >"$env_path"

echo "CONFIG=\"$cli_client_dir/rsm-conf.json\"" >"$env_dir"
echo "LOG=\"$log_file\"" >>"$env_dir"

echo "Building the binary for rsm..." && cargo build -q --release
echo "Finished building!"

# Copy the built binary to /usr/local/bin
sudo cp "./target/release/rsm" /usr/local/bin/

cargo clean -q

echo ""
echo "$(tput setaf 4)You can now use the command rsm!$(tput sgr0)"
echo "$(tput setaf 4)Try running rsm --help$(tput sgr0)"
