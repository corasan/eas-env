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

# Add the directory to the PATH
export PATH="$HOME/.local/bin:$PATH"