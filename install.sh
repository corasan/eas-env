#!/bin/bash

# Get the latest release version
latest_version=$(curl -s https://api.github.com/repos/corasan/eas-env/releases/latest | grep tag_name | cut -d '"' -f 4)

# Download the release binary
curl -L -o eas-env "https://github.com/corasan/eas-env/releases/download/$latest_version/eas-env"

# Make the binary executable
chmod +x eas-env

# Create the target directory if it doesn't exist
mkdir -p $HOME/.local/bin


# Move the binary to a directory in the PATH
mv eas-env $HOME/.local/bin/

# Detect the user's shell
shell=$(basename "$SHELL")

# Set the shell configuration file based on the detected shell
if [ "$shell" == "zsh" ]; then
    shell_config="$HOME/.zshrc"
elif [ "$shell" == "bash" ]; then
    shell_config="$HOME/.bashrc"
else
    echo "Unsupported shell: $shell"
    exit 1
fi

# Check if the PATH modification line already exists in the shell configuration file
if grep -qxF 'export PATH="$HOME/.local/bin:$PATH"' "$shell_config"; then
    echo "PATH modification already exists in $shell_config"
else
    # Add $HOME/.local/bin to the PATH
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$shell_config"
    source "$shell_config"
    echo "Added $HOME/.local/bin to PATH in $shell_config"
fi