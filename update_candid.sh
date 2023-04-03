#!/bin/bash

# set path to kybra package
KYBRA_PATH=$1

# loop through subdirectories of examples
for dir in examples/*; do
  # check if dfx.json file exists in directory
  if [ -f "$dir/dfx.json" ]; then
    # deactivate current virtual environment if running
    deactivate 2>/dev/null
    # start a new virtual environment
    ~/.pyenv/versions/3.10.7/bin/python -m venv venv
    # activate new virtual environment
    source venv/bin/activate
    # install kybra package
    pip install $KYBRA_PATH
    # start dfx replica in background
    dfx start --host 127.0.0.1:8000 &
    # save PID of dfx process
    PID=$!
    # deploy project
    dfx deploy
    # kill dfx process
    kill $PID
  fi
done
