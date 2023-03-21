#!/bin/bash

# Instructions: Run this script inside of the examples direcotry

# Find all package.json files and loop through them
find . -name "package.json" -type f -print0 | while IFS= read -r -d '' package_json; do
    # Check if the azle dependency is present
    if grep -q "\"azle\"" "$package_json"; then
        echo "Found azle dependency in $package_json"
        # Uninstall azle and install the demergent-labs version
        dir=$(dirname "$package_json")
        cd "$dir"
        npm uninstall azle
        npm install https://github.com/demergent-labs/azle
        cd - > /dev/null
    fi
done
