#!/bin/bash
# ----------------------------------------------------------------------------------------------------
# | This script creates the base for the log (if the app doesn't find the log file it's gonna break) |
# | builds the binary and puts the binary into a path where it can be ran from anywhere              |
# ----------------------------------------------------------------------------------------------------

# Find the directory containing cli_client and its subdir log
cli_client_dir=$(find ~/ -type d -name "cli_client" 2>/dev/null)
log_dir="$cli_client_dir/log"

# Create the log directory if it doesn't exist
mkdir -p "$log_dir"

# Create the log file rsm-log.log inside log directory
log_file="$log_dir/rsm-log.log"
touch "$log_file"

# Run cargo build --release
cd "$cli_client_dir" || exit
echo "Building the binary for rsm..." && cargo build -q --release
echo "Finished building!"

# Copy the built binary to /usr/local/bin
sudo cp "./target/release/rsm" /usr/local/bin/

cargo clean -q

echo ""
echo -e "\e[34mYou can now use the command rsm!\e[0m"
echo -e "\e[34mTry running rsm --help\e[0m"
