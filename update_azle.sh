#!/bin/bash

# Instructions: Run this script inside of the examples direcotry

# Traverse the directory structure and find all package.json files, excluding node_modules directories
while IFS= read -r -d '' dir; do
    package_json="$dir/package.json"
    if [[ -f "$package_json" ]] && ! [[ "$package_json" == *"node_modules"* ]] && grep -q "\"azle\"" "$package_json"; then
        echo "Found azle dependency in $package_json"
        # Uninstall azle and install the demergent-labs version
        (cd "$dir" && npm uninstall azle && npm install https://github.com/demergent-labs/azle)
    fi
done < <(find . -type d -not -path '*/node_modules/*' -print0)
