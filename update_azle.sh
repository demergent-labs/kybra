#!/bin/bash

# Instructions: Run this script inside of the examples direcotry

# function to check if azle is a dependency in package.json
check_azle_dependency() {
    if grep -q "\"azle\":" "$1"; then
        return 0
    else
        return 1
    fi
}

updated=0
updated_folders=()
checked=0

# loop through all folders that are not inside node_modules
while IFS= read -r -d '' dir; do
    if [[ "$dir" != *"node_modules"* ]] && [[ -f "$dir/package.json" ]]; then
        if check_azle_dependency "$dir/package.json"; then
            cd "$dir"
            npm uninstall azle && npm install https://github.com/demergent-labs/azle#test_case_converter
            cd - >/dev/null
            ((updated++))
            updated_folders+=("$dir")
        fi
        ((checked++))
        echo "Checked $checked packages..."
    fi
done < <(find . -type d -print0)

# generate report
if [[ "$updated" -eq 0 ]]; then
    echo "No packages with azle dependency found."
else
    echo "Updated azle in $updated package(s):"
    printf '%s\n' "${updated_folders[@]}"
fi
