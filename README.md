<div align="center">
    <a href="https://github.com/demergent-labs/kybra" target="_blank" rel="noopener noreferrer">
        <img height="150" src="https://raw.githubusercontent.com/demergent-labs/kybra/main/logo/logo.svg" alt="kybra logo">
    </a>
</div>

# Kybra (Alpha)

Python CDK for the [Internet Computer](https://internetcomputer.org/).

## Disclaimer

Please exercise caution when using Kybra. It is alpha software that you use at your own risk and according to the terms of this [MIT license](/LICENSE).

Demergent Labs may officially recommend Kybra for production use when at least the following have occurred:

-   [ ] Many example-based unit/integration tests
-   [ ] Feature parity with the Azle, Motoko, and Rust CDKs
-   [ ] Extensive automated benchmarking
-   [ ] Extensive automated property testing
-   [ ] Multiple independent security reviews/audits

## Discussion

Feel free to open issues or join us in the [Discord channel](https://discord.com/channels/748416164832608337/1019372359775440988).

## Documentation

Most of Kybra's documentation is currently found in this README. The Kybra Book, similar to [Sudograph's](https://i67uk-hiaaa-aaaae-qaaka-cai.raw.ic0.app/), will later be hosted on the Internet Computer.

-   [Examples](/examples)
-   [Installation](#installation)
-   [Deployment](#deployment)
-   [Supported Features](#supported-features)
-   [Contributing](#contributing)

### Installation

Follow instructions exactly as stated below to avoid issues.

You should be using a \*nix environment (Linux, Mac OS, [WSL](https://learn.microsoft.com/en-us/windows/wsl/install)) with bash and have the following installed on your system:

-   [Python 3.10.7](#python-3.10.7)
-   [Rust](#rust)
-   [dfx 0.12.0](#dfx)
-   [Python VS Code Extension](#python-vs-code-extension)

#### Python 3.10.7

It is highly recommended to install Python 3.10.7 using [pyenv](https://github.com/pyenv/pyenv). To do so, use the [pyenv installer](https://github.com/pyenv/pyenv-installer):

```bash
# install pyenv
curl https://pyenv.run | bash

# install Python 3.10.7
~/.pyenv/bin/pyenv install 3.10.7
```

#### Rust

Run the following command to install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you already have Rust installed, make sure you are up to date:

```bash
rustup update
```

The following are some common Rust installation issues:

-   Ubuntu
    -   error: linker cc not found (sudo apt install build-essential)
    -   is cmake not installed? (sudo apt install cmake)

#### dfx

Run the following command to install dfx 0.12.0:

```bash
DFX_VERSION=0.12.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

#### Python VS Code Extension

It is highly recommended to use VS Code and to install the [Microsoft Python extension](https://marketplace.visualstudio.com/items?itemName=ms-python.python) to get full type checking support from within the editor:

##### Extension

```
VS Code -> Preferences -> Extensions -> Search for Python by Microsoft and install it
```

##### Set the interpreter to Python 3.10.7

Use the `Python: Select Interpreter` command from the `Command Palette`:

```
(Ctrl+Shift+P) then search for and select "Python: Select Interpreter"
Select or enter this path to the interpreter: ~/.pyenv/versions/3.10.7/bin/python
```

##### Set python.analysis.typeCheckingMode

Set the setting `python.analysis.typeCheckingMode` to `strict`:

```
VS Code -> Preferences -> Settings -> Search for python.analysis.typeCheckingMode and set it to strict
```

##### Set python.analysis.extraPaths

Set the setting `python.analysis.extraPaths` with the path to your `site-packages` directory `.dfx/kybra/venv/lib/python3.10/site-packages`:

```
VS Code -> Preferences -> Settings -> Search for python.analysis.extraPaths and add .dfx/kybra/venv/lib/python3.10/site-packages
```

#### Kybra

Follow these steps to create a Kybra project. The steps below assume a project called `hello_world`:

1. Create a directory for your project (`mkdir hello_world && cd hello_world`)
2. Create the virtual environment: (`~/.pyenv/versions/3.10.7/bin/python -m venv .dfx/kybra/venv`)
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

Your `src/main.py` file should look like this:

```python
from kybra import query

@query
def hello_world() -> str:
    return 'Hello world!'
```

You are now ready to [deploy your application](#deployment).

### Deployment

Please keep in mind that you must deploy from within the `virtualenv` that you set up earlier: `source .dfx/kybra/venv/bin/activate`.

#### Local Deployment

Start up an IC replica and deploy. The first deploy will likely take multiple minutes as it downloads and compiles many Rust dependencies. Subsequent deploys should be much quicker:

```bash
# Open a terminal and navigate to your project's root directory, then run the following command to start a local IC replica
dfx start

# Alternatively to the above command, you can run the replica in the background
dfx start --background

# If you are running the replica in the background, you can run the following commands within the same terminal as the dfx start --background command
# If you are not running the replica in the background, then open another terminal and run the following commands from the root directory of your project

# This source command is only required once per terminal session
source .dfx/kybra/venv/bin/activate

dfx deploy
```

You can then interact with your canister like any other canister written with Azle, Motoko, or Rust. For more information about calling your canister using `dfx`, see [here](https://smartcontracts.org/docs/developers-guide/cli-reference/dfx-canister.html#_dfx_canister_call).

#### Live Deployment

Deploying to the live Internet Computer generally only requires adding the `--network ic` option to the deploy command: `dfx deploy --network ic`. This assumes you already have converted ICP into cycles appropriately. See [here](https://smartcontracts.org/docs/quickstart/4-quickstart.html) for more information on getting ready to deploy to production.

### Supported Features

Kybra Alpha is very limited in features. A good guide is that you can only do what is exported from the [kybra module](/kybra/__init__.py). Looking at the [examples](/examples) should be very helpful as well.

Keep in mind that Kybra requires the use of [Python typing](https://docs.python.org/3/library/typing.html).

You can create `query` and `update` functions.

You can import and use any of these primitive types (`bool`, `int`, and `str` are built-in thus need no import): `int`, `int64`, `int32`, `int16`, `int8`, `nat`, `nat64`, `nat32`, `nat16`, `nat8`, `float64`, `float32`, `text`, `str`, `bool`.

You can import modules created locally or installed with `pip`. If you create a module in a subdirectory, it must have an `__init__.py` file.

#### stdlib

There is limited support for the `stdlib`. The following modules may be supported as far as [RustPython](https://github.com/RustPython/RustPython) or the IC support them:

-   array
-   binascii
-   bisect
-   bz2
-   cmath
-   contextvars
-   csv
-   dis
-   faulthandler
-   fcntl
-   gc
-   grp
-   hashlib
-   json
-   math
-   mmap
-   multiprocessing
-   \_posixsubprocess
-   pyexpat
-   struct
-   random
-   re
-   resource
-   \_scproxy
-   select
-   socket
-   ssl
-   statistics
-   syslog
-   termios
-   unicodedata
-   uuid
-   zlib

### Contributing

Not currently taking contributions, but definitely taking issues and questions. Please allow time for initial code architecture and governance/legal/token work to be put in place. Kybra will most likely have a license extension [similar to Azle's](https://github.com/demergent-labs/azle/blob/main/LICENSE_EXTENSION.md).

#### Publishing to PyPI

```bash
# prepare on new machine
~/.pyenv/versions/3.10.7/bin/python -m pip install --upgrade build
~/.pyenv/versions/3.10.7/bin/python -m pip install --upgrade twine

# build
~/.pyenv/versions/3.10.7/bin/python -m build

# upload
~/.pyenv/versions/3.10.7/bin/python -m twine upload --skip-existing dist/*
```

#### Local Development

Install Kybra with pip from the repository. For example, when working from an example in the examples directory:

```bash
pip install ../.. && dfx deploy
```
