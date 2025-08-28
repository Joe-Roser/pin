#!/bin/bash

INST_D="$HOME/.pin"

# If install exitst, remove without deleting store.bin
if [ -f "$INST_D/pin.sh" ]; then
	echo "Deleting existing installation"
	rm "$INST_D/pin.sh"
fi
if [ -f "$INST_D/_pin" ]; then
	rm "$INST_D/_pin"
fi

mkdir -p "$INST_D"

# Install files
echo "Installing pin..."
cp "./pin.sh" "$INST_D/"
cp "./target/release/_pin" "$INST_D/"

# Add pin to PATH if not already there
if ! grep -q "\$HOME/.pin/pin.sh" ~/.bashrc; then
	echo "Adding pin.sh to PATH in ~/.bashrc"
	echo "#pin\n\rsource \$HOME/.pin/pin.sh" >> ~/.bashrc
	echo "Please restart your terminal or run 'source ~/.bashrc' to apply changes."
else
	echo "pin.sh is already in PATH."
fi

echo "Installation complete."


