#!/bin/bash

# For a given directory loop through all agent folders and comment out the last line in the index.js file.
cd $1
for directory in */; do
    echo "Processing $directory"
    STORED_LAST_LINE=$(tail --lines=1 "$directory"/index.js)
    echo "The saved text is: $STORED_LAST_LINE"
    sed -i '$ d' "$directory"/index.js
    echo "// $STORED_LAST_LINE" >> "$directory"/index.js
done
