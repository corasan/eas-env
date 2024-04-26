#!/bin/bash

# Check if the binary already exists in the target directory
if [ -f "./node_modules/.bin/eas-env" ]; then
    echo "eas-env binary already exists in ./node_modules/.bin"
else
    # Get the latest release version
    latest_version=$(curl -s https://api.github.com/repos/corasan/eas-env/releases/latest | grep tag_name | cut -d '"' -f 4)

    # Download the release binary
    curl -L -o eas-env "https://github.com/corasan/eas-env/releases/download/$latest_version/eas-env"

    # Make the binary executable
    chmod +x eas-env

    # Move the binary to the ./node_modules/.bin directory
    mv eas-env ./node_modules/.bin/
fi