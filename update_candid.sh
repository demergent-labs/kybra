#!/bin/bash

# Usage:
# cd examples
# ../update_candid
# cd motoko_examples
# ../update_candid ../..

# set path to kybra package. Default to ../ (for when run from example dir)
KYBRA_PATH="${1:-../}"

upgrade_candid()
{
  # deactivate current virtual environment if running
  deactivate 2>/dev/null
  # start a new virtual environment
  ~/.pyenv/versions/3.10.7/bin/python -m venv venv
  # activate new virtual environment
  source venv/bin/activate
  # install kybra package
  # pip install $KYBRA_PATH
  pip install ../../ # TODO change this as appropriate
  # start dfx replica in background
  # dfx start --host 127.0.0.1:8000 --clean &

  # sleep 7

  dfx canister create --all
  dfx build
  deactivate 2>/dev/null
  # killall dfx
}

upgrade_all()
{

for dir in */; do
  echo "Looking at directory: $dir"
  # check if dfx.json file exists in directory
  if [ -f "$dir/dfx.json" ]; then
    echo "Processing directory: $dir"
    cd $dir
    upgrade_candid
    # sleep 5
    cd ..
  fi
done
}

upgrade_all
