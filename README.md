# Kybra (Alpha)

Python CDK for the [Internet Computer](https://internetcomputer.org/).

## Disclaimer

Please exercise caution when using Kybra. It is alpha software that you use at your own risk and according to the terms of this [MIT license](/LICENSE).

Demergent Labs may officially recommend Kybra for production use when at least the following have occurred:

- [ ] Many example-based unit/integration tests
- [ ] Feature parity with the Azle, Motoko, and Rust CDKs
- [ ] Extensive automated benchmarking
- [ ] Extensive automated property testing
- [ ] Multiple independent security reviews/audits

## Discussion

Feel free to open issues or join us in the [Discord channel](https://discord.com/channels/748416164832608337/1019372359775440988).

## Documentation

Most of Kybra's documentation is currently found in this README. The Kybra Book, similar to [Sudograph's](https://i67uk-hiaaa-aaaae-qaaka-cai.raw.ic0.app/), will later be hosted on the Internet Computer.

- [Examples](/examples)
- [Installation](#installation)
- [Deployment](#deployment)
- [Supported Features](#supported-features)
- [Contributing](#contributing)

### Installation

You should be using a \*nix environment (Linux, Mac OS, [WSL](https://learn.microsoft.com/en-us/windows/wsl/install)) with bash and have the following installed on your system:

- [Python 3](https://www.python.org/downloads/)
- [virtualenv](#virtualenv)
- [Rust](#rust)
- [dfx](#dfx)
- [Python Extension](#python-extension)

#### virtualenv

Run the following command to install virtualenv:

```python
python3 -m pip install virtualenv
```

#### Rust

Run the following command to install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The following are some common Rust installation issues:

- Ubuntu
  - error: linker cc not found (sudo apt install build-essential)
  - is cmake not installed? (sudo apt install cmake)

#### dfx

Run the following command to install dfx 0.12.0-beta.2:

```bash
DFX_VERSION=0.12.0-beta.2 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

#### Kybra

Follow these steps to create a Kybra project. The steps below assume a project called `hello_world`:

1. Create a directory for your project (`mkdir hello_world && cd hello_world`)
2. Create the virtual environment: (`python3 -m virtualenv .dfx/kybra/venv`)
3. Activate the virtual environment: (`source .dfx/kybra/venv/bin/activate`)
4. Install Kybra (`pip install kybra`)
5. Create a `dfx.json` file (`touch dfx.json`)
6. Create a directory and an entry Python file for your canister (`mkdir src && touch src/main.py`)

Your `dfx.json` file should look like this:

```json
{
  "canisters": {
    "hello_world": {
      "type": "custom",
      "build": "python -m kybra hello_world src/main.py src/main.did",
      "candid": "src/main.did",
      "wasm": ".dfx/kybra/hello_world/target/wasm32-unknown-unknown/release/hello_world.wasm.gz"
    }
  }
}
```

The `build` command is `python -m kybra` followed by the canister name, the relative path to your entry Python file, and the relative path where you would like your Candid file to be generated.

Your `main.py` file should look like this:

```python
from kybra import query

@query
def hello_world() -> str:
    return 'Hello world!'
```

You are now ready to [deploy your application](#deployment).

#### Python Extension

It is highly recommended to install the [Microsoft Python extension](https://marketplace.visualstudio.com/items?itemName=ms-python.python) to get type errors reported in VS Code:

```
VS Code -> File -> Preferences -> Extensions -> Search for Python by Microsoft
```

Enable the setting `python.analysis.typeCheckingMode`:

```
VS Code -> File -> Preferences -> Settings -> Search for python.analysis.typeCheckingMode and set it to strict
```

If VS Code shows errors when you try to import from `kybra` or local files, then also set `python.analysis.extraPaths` with the path to your source directory and the path `.dfx/kybra/venv/lib/python[your-version-here]/site-packages`.

```
VS Code -> File -> Preferences -> Settings -> Search for python.analysis.extraPaths and add the directories stated above
```

### Deployment

#### Local Deployment

Start up an IC replica and deploy. The first deploy will likely take multiple minutes as it downloads and compiles many Rust dependencies. Subsequent deploys should be much quicker:

```bash
# Open a terminal and navigate to your project's root directory, then run the following command to start a local IC replica
dfx start

# Alternatively to the above command, you can run the replica in the background
dfx start --background

# If you are running the replica in the background, you can run this command within the same terminal as the dfx start --background command
# If you are not running the replica in the background, then open another terminal and run this command from the root directory of your project

dfx deploy
```

You can then interact with your canister like any other canister written with Azle, Motoko, or Rust. For more information about calling your canister using `dfx`, see [here](https://smartcontracts.org/docs/developers-guide/cli-reference/dfx-canister.html#_dfx_canister_call).

#### Live Deployment

Deploying to the live Internet Computer generally only requires adding the `--network ic` option to the deploy command: `dfx deploy --network ic`. This assumes you already have converted ICP into cycles appropriately. See [here](https://smartcontracts.org/docs/quickstart/4-quickstart.html) for more information on getting ready to deploy to production.

### Supported Features

Kybra Alpha is very limited in features. A good guide is that you can only do what is exported from the [kybra module](/kybra/__init__.py).

You can create `query` and `update` functions.

You can import and use any of these primitive types (`bool`, `int`, and `str` are built-in thus need no import): `int`, `int64`, `int32`, `int16`, `int8`, `nat`, `nat64`, `nat32`, `nat16`, `nat8`, `float64`, `float32`, `text`, `str`, `bool`.

You can create and import local modules defined in your canister's root directory (the directory of your entry Python file). If you create subdirectories, they must have a `__init__.py` file.

There is no support for `stdlib`. There is no support for installing external modules (`pip install` just won't work).

### Contributing

Not currently taking contributions, but definitely taking issues and questions. Please allow time for initial code architecture and governance/legal/token work to be put in place. Kybra will most likely have a license extension [similar to Azle's](https://github.com/demergent-labs/azle/blob/main/LICENSE_EXTENSION.md).

#### Publishing to PyPI

```bash
# do the following within the virtualenv

# prepare on new machine
python3 -m pip install --upgrade build
python3 -m pip install --upgrade twine

# build
python3 -m build

# upload
python3 -m twine upload dist/*
```

#### Local Development

Install Kybra with pip from the repository. For example, when working from an example in the examples directory:

```bash
pip install ../.. && dfx deploy
```
