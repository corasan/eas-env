#!/bin/bash

# Get the latest release version
latest_version=$(curl -s https://api.github.com/repos/corasan/eas-env/releases/latest | grep tag_name | cut -d '"' -f 4)

# Download the release binary
curl -L -o eas-env "https://github.com/corasan/eas-env/releases/download/$latest_version/eas-env"

# Make the binary executable
chmod +x eas-env

# Move the binary to a directory in the PATH
sudo mv eas-env /usr/local/bin/