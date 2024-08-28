#!/bin/bash

echo "Building project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

if [ ! -f target/release/sftm ]; then
    echo "Error: target/release/sftm not found. Exiting."
    exit 1
fi

TEMPLATE_TARGET_PATH="/home/$USER/.local/share/LICENSER/"
mkdir -p "$TEMPLATE_TARGET_PATH"
mv Templates/ "$TEMPLATE_TARGET_PATH"

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected macOS. Copying sftm to /usr/local/bin..."
    sudo cp target/release/sftm /usr/local/bin/
    INSTALL_PATH="/usr/local/bin/sftm"
else
    echo "Copying sftm to /usr/bin..."
    sudo cp target/release/sftm /usr/bin/
    INSTALL_PATH="/usr/bin/sftm"
fi

if [ $? -ne 0 ]; then
    echo "Failed to copy sftm to $INSTALL_PATH. Exiting."
    exit 1
fi

echo "sftm has been installed.🔥"