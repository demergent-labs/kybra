#!/bin/bash

for d in */ ; do
  if [ -f "${d}dfx.json" ]; then
    echo "Processing ${d}..."

    # deactivate any existing Python virtual environment
    deactivate &>/dev/null || true

    # start a new Python virtual environment
    ~/.pyenv/versions/3.10.7/bin/python -m venv venv
    source venv/bin/activate

    # install kybra package
    pip install "$1"

    # start dfx replica
    dfx start --host 127.0.0.1:8000 &
    while ! output=$(dfx identity list) || ! [[ "$output" =~ "Dashboard: http://localhost:[0-9]{4,}/_/" ]]; do
      echo "Waiting for dfx replica to start up... ($output)"
      sleep 1
    done

    # deploy project
    dfx deploy

    # kill dfx replica process
    pkill -f "dfx start"

    # deactivate Python virtual environment
    deactivate

    echo "${d} processed."
  fi
done
