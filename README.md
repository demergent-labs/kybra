<div align="center">
    <a href="https://github.com/demergent-labs/kybra" target="_blank" rel="noopener noreferrer">
        <img height="150" src="https://raw.githubusercontent.com/demergent-labs/kybra/main/logo/logo.svg" alt="kybra logo">
    </a>
</div>
</br>
<div align="center">
    <a href="https://github.com/demergent-labs/kybra/actions/workflows/test.yml?query=branch%3Amain">
        <img src="https://github.com/demergent-labs/kybra/actions/workflows/test.yml/badge.svg" alt="Coverage Status">
    </a>
    <a href="https://badge.fury.io/py/kybra"><img src="https://badge.fury.io/py/kybra.svg" alt="PyPI version"></a>
</div>

# Kybra (Beta)

Python CDK for the [Internet Computer](https://internetcomputer.org/).

## Disclaimer

Please consider the [security section](#security).

## Discussion

Feel free to open issues or join us in the [Discord channel](https://discord.com/channels/748416164832608337/1019372359775440988).

## Documentation

Most of Kybra's documentation is currently found in this README. The Kybra Book, similar to [Sudograph's](https://i67uk-hiaaa-aaaae-qaaka-cai.raw.ic0.app/), will later be published.

-   [Examples](/examples)
-   [Installation](#installation)
-   [Deployment](#deployment)
-   [Canisters](#canisters)
-   [Canister Methods](#canister-methods)
-   [Candid Types](#candid-types)
-   [Canister APIs](#canister-apis)
-   [Call APIs](#call-apis)
-   [Stable Memory](#stable-memory)
-   [Python stdlib](#python-stdlib)
-   [Python External Packages](#python-external-packages)
-   [Security](#security)
-   [Roadmap](#roadmap)
-   [Gotchas and Caveats](#gotchas-and-caveats)
-   [Contributing](#contributing)

### Installation

Follow instructions exactly as stated below to avoid issues.

You should be using a \*nix environment (Linux, Mac OS, [WSL](https://learn.microsoft.com/en-us/windows/wsl/install)) with bash and have the following installed on your system:

-   [Python 3.10.7](#python-3.10.7)
-   [Rust](#rust)
-   [dfx 0.12.0](#dfx)
-   [Python VS Code Extension](#python-vs-code-extension)
-   [Common Errors](#common-errors)

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

#### Common Errors

These are common errors or problems that you might run into while getting everything installed.

Error:

```
No such file or directory: '.../.cargo/.bin/ic-cdk-optimizer'
```

Resolution:

-   You may need to manually install `ic-cdk-optimizer`:

```
cargo install ic-cdk-optimizer
```

Error (Ubuntu):

```
linker cc not found
```

Resolution:

```
sudo apt install build-essential
```

Error (Ubuntu):

```
is cmake not installed?
```

Resolution:

```
sudo apt install cmake
```

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

You can then interact with your canister like any other canister written with Motoko, Rust, or Azle. For more information about calling your canister using `dfx`, see [here](https://internetcomputer.org/docs/current/references/cli-reference/dfx-canister#dfx-canister-call).

dfx commands for the [query example](/examples/query):

```bash
dfx canister call query simple_query
# The result is: ("This is a query function")
```

dfx commands for the [update example](/examples/update):

```bash
dfx canister call update simple_update '("Why hello there")'
# The result is: ()

dfx canister call update get_current_message
# The result is: ("Why hello there")
```

dfx commands for the [simple_erc20 example](/examples/simple_erc20):

```bash
dfx canister call simple_erc20 initialize_supply '("Token", "0", "TOKEN", 1_000_000)'
# The result is: (true)

dfx canister call simple_erc20 name
# The result is: ("Token")

dfx canister call simple_erc20 ticker
# The result is: ("TOKEN")

dfx canister call simple_erc20 total_supply
# The result is: (1_000_000 : nat64)

dfx canister call simple_erc20 balance '("0")'
# The result is: (1_000_000 : nat64)

dfx canister call simple_erc20 transfer '("0", "1", 100)'
# The result is: (true)
```

#### Live Deployment

Deploying to the live Internet Computer generally only requires adding the `--network ic` option to the deploy command: `dfx deploy --network ic`. This assumes you already have converted ICP into cycles appropriately. See [here](https://internetcomputer.org/docs/current/developer-docs/quickstart/network-quickstart#before-you-begin) for more information on getting ready to deploy to production.

### Canisters

More information:

-   https://internetcomputer.org/docs/current/concepts/canisters-code
-   https://wiki.internetcomputer.org/wiki/Canisters_(dapps/smart_contracts)

In many ways developing canisters with Kybra is similar to any other Python project. To see what canister source code looks like, see the [examples](/examples).

A canister is the fundamental application unit on the Internet Computer. It contains the code and state of your application. When deployed to the Internet Computer, your canister essentially becomes an everlasting process.

Users of your canister interact with it through RPC calls performed using HTTP requests. These calls will hit your canister's `@query` and `@update` methods. These methods, with their parameter and return types, are the interface to your canister.

Kybra allows you to write canisters while embracing much of what the Python ecosystem has to offer.

### Canister Methods

-   [init](#init)
-   [pre upgrade](#pre-upgrade)
-   [post upgrade](#post-upgrade)
-   [inspect message](#inspect-message)
-   [heartbeat](#heartbeat)
-   [update](#update)
-   [query](#query)
-   [http_request and http_request_update](#http_request-and-http_request_update)

#### init

Examples:

-   [func_types](/examples/func_types)
-   [init](/examples/init)
-   [persistent-storage](/examples/motoko_examples/persistent-storage)
-   [pre_and_post_upgrade](/examples/pre_and_post_upgrade)
-   [whoami](/examples/motoko_examples/whoami)

```python
from kybra import ic, init

@init
def init_():
    ic.print('This runs once when the canister is first initialized')
```

#### pre upgrade

Examples:

-   [pre_and_post_upgrade](/examples/pre_and_post_upgrade)
-   [stable_storage](/examples/stable_storage)

```python
from kybra import ic, pre_upgrade

@pre_upgrade
def pre_upgrade_():
    ic.print('This runs before every canister upgrade')
```

#### post upgrade

Examples:

-   [pre_and_post_upgrade](/examples/pre_and_post_upgrade)
-   [stable_storage](/examples/stable_storage)
-   [whoami](/examples/motoko_examples/whoami)

```python
from kybra import ic, post_upgrade

@post_upgrade
def post_upgrade_():
    ic.print('This runs after every canister upgrade')
```

#### inspect message

Examples:

-   [inspect_message](/examples/inspect_message)

```python
from kybra import ic, inspect_message, update

@inspect_message
def inspect_message_():
    ic.print('this runs before executing update calls')

    if ic.method_name() == 'accessible':
        ic.accept_message()
        return

    if ic.method_name() == 'inaccessible':
        return

    raise Exception(f'Method "{ic.method_name()}" is not allowed')

@update
def accessible() -> bool:
    return True

@update
def inaccessible() -> bool:
    return False

@update
def also_inaccessible() -> bool:
    return False
```

#### heartbeat

Examples:

-   [heartbeat](/examples/heartbeat)

```python
from kybra import heartbeat, ic

@heartbeat
def heartbeat_():
    ic.print('this runs ~1 time per second')
```

#### update

Examples:

-   [update](/examples/update)
-   [key_value_store](/examples/key_value_store)

More information:

-   https://internetcomputer.org/docs/current/concepts/canisters-code#query-and-update-methods

Update methods expose public callable functions that are writable. All state changes will be persisted after the function call completes.

Update calls go through consensus and thus return very slowly (a few seconds) relative to query calls. This also means they are more secure than query calls unless [certified variables](https://internetcomputer.org/how-it-works/response-certification/) are used in conjunction with the query call.

To create an update method, simply add the `update` decorator to your function.

```python
from kybra import query, update

current_message: str = ''

@query
def query_() -> str:
    return current_message

@update
def update(message: str):
    global current_message
    current_message = message
```

#### query

Examples:

-   [query](/examples/query)
-   [update](/examples/update)
-   [simple_user_accounts](/examples/simple_user_accounts)

More information:

-   https://internetcomputer.org/docs/current/concepts/canisters-code#query-and-update-methods

Query methods expose public callable functions that are read-only. All state changes will be discarded after the function call completes.

Query calls do not go through consensus and thus return very quickly relative to update calls. This also means they are less secure than update calls unless [certified variables](https://internetcomputer.org/how-it-works/response-certification/) are used in conjunction with the query call.

To create a query method, simply add the `query` decorator to your function.

```python
from kybra import query

@query
def query_() -> str:
    return 'This is a query function'
```

#### http_request and http_request_update

Examples:

-   [http_counter](/examples/motoko_examples/http_counter)

```python
from kybra import blob, Func, ic, nat, nat16, opt, query, Query, update, variant

class HttpRequest(Record):
    method: str
    url: str
    headers: list['HeaderField']

class HttpResponse(Record):
    status_code: nat16
    headers: list['HeaderField']
    body: blob
    streaming_strategy: opt[StreamingStrategy]
    upgrade: opt[bool]

HeaderField = tuple[str, str]

class StreamingStrategy(Variant, total=False):
    Callback: 'CallbackStrategy'

class CallbackStrategy(Record):
    callback: 'Callback'
    token: 'Token'

Callback: TypeAlias = Func(Query[[t: 'Token'], 'StreamingCallbackHttpResponse'])

class Token(Record):
    arbitrary_data: str

class StreamingCallbackHttpResponse(Record):
    body: blob
    token: opt[Token]

@query
def http_request(req: HttpRequest) -> HttpResponse:
    return {
        'status_code': 200,
        'headers': [('content-type', 'text/plain')],
        'body': bytes(),
        'streaming_strategy': None,
        'upgrade': True
    }

@update
def http_request_update(req: HttpRequest) -> HttpResponse:
    return {
        'status_code': 200,
        'headers': [('content-type', 'text/plain')],
        'body': bytes(),
        'streaming_strategy': None,
        'upgrade': None
    }
```

### Candid Types

Examples:

-   [primitive_types](/examples/primitive_types)
-   [complex_types](/examples/complex_types)

[Candid](https://internetcomputer.org/docs/current/developer-docs/build/candid/candid-intro) is an interface description language created by [DFINITY](https://dfinity.org/). It defines interfaces between services (in our context canisters), allowing canisters and clients written in various languages to easily interact with each other.

Much of what Kybra is doing under-the-hood is translating Python code into various formats that Candid understands (for example Kybra will generate a Candid `.did` file from your Python code). To do this your Python code must use various Kybra-provided types.

Please note that these types are only needed in specific locations in your code, including but not limited to the following areas:

-   `@query`, `@update`, `@init`, and `@post_upgrade` method parameters and return types
-   External `Canister` method declaration parameters and return types

Basically, you only need to write using type hints and use the Kybra types when Candid serialization or deserialization is necessary. You could write the rest of your application in plain Python if you'd like.

Data types:

-   [text](#text)
-   [blob](#blob)
-   [nat](#nat)
-   [nat64](#nat64)
-   [nat32](#nat32)
-   [nat16](#nat16)
-   [nat8](#nat8)
-   [int](#int)
-   [int64](#int64)
-   [int32](#int32)
-   [int16](#int16)
-   [int8](#int8)
-   [float64](#float64)
-   [float32](#float32)
-   [bool](#bool)
-   [null](#null)
-   [vec](#vec)
-   [opt](#opt)
-   [record](#record)
-   [variant](#variant)
-   [func](#func)
-   [service](#service)
-   [principal](#principal)
-   [reserved](#reserved)
-   [empty](#empty)

#### text

The Python type `str` and the Kybra type `text` both correspond to the [Candid type text](https://internetcomputer.org/docs/current/references/candid-ref#type-text) and will become a [Python str](https://docs.python.org/3/library/stdtypes.html#textseq) at runtime.

Python:

```python
from kybra import ic, query

@query
def get_string() -> str:
    return 'Hello world!'

@query
def print_string(string: str) -> str:
    ic.print(type(string))
    return string
```

Candid:

```python
service: {
    "get_string": () -> (text) query;
    "print_string": (text) -> (text) query;
}
```

#### blob

The Kybra type `blob` corresponds to the [Candid type blob](https://internetcomputer.org/docs/current/references/candid-ref#type-blob) and will become a [Python bytes](https://docs.python.org/3/library/stdtypes.html#bytes) at runtime.

Python:

```Python
from kybra import blob, ic, query

@query
def get_blob() -> blob:
    return bytes([68, 73, 68, 76, 0, 0])

@query
def print_blob(blob: blob) -> blob:
    ic.print(type(blob))
    return blob
```

Candid:

```python
service: {
    "get_blob": () -> (blob) query;
    "print_blob": (blob) -> (blob) query;
}
```

#### nat

The Kybra type `nat` corresponds to the [Candid type nat](https://internetcomputer.org/docs/current/references/candid-ref#type-nat) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat, query

@query
def get_nat() -> nat:
    return 340_282_366_920_938_463_463_374_607_431_768_211_455

@query
def print_nat(nat: nat) -> nat:
    ic.print(type(nat))
    return nat
```

Candid:

```python
service: {
    "get_nat": () -> (nat) query;
    "print_nat": (nat) -> (nat) query;
}
```

#### nat64

The Kybra type `nat64` corresponds to the [Candid type nat64](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat64, query

@query
def get_nat64() -> nat64:
    return 18_446_744_073_709_551_615

@query
def print_nat64(nat64: nat64) -> nat64:
    ic.print(type(nat64))
    return nat64
```

Candid:

```python
service: {
    "get_nat64": () -> (nat64) query;
    "print_nat64": (nat64) -> (nat64) query;
}
```

#### nat32

The Kybra type `nat32` corresponds to the [Candid type nat32](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat32, query

@query
def get_nat32() -> nat32:
    return 4_294_967_295

@query
def print_nat32(nat32: nat32) -> nat32:
    ic.print(type(nat32))
    return nat32
```

Candid:

```python
service: {
    "get_nat32": () -> (nat32) query;
    "print_nat32": (nat32) -> (nat32) query;
}
```

#### nat16

The Kybra type `nat16` corresponds to the [Candid type nat16](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat16, query

@query
def get_nat16() -> nat16:
    return 65_535

@query
def print_nat16(nat16: nat16) -> nat16:
    ic.print(type(nat16))
    return nat16
```

Candid:

```python
service: {
    "get_nat16": () -> (nat16) query;
    "print_nat16": (nat16) -> (nat16) query;
}
```

#### nat8

The Kybra type `nat8` corresponds to the [Candid type nat8](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat8, query

@query
def get_nat8() -> nat8:
    return 255

@query
def print_nat8(nat8: nat8) -> nat8:
    ic.print(type(nat8))
    return nat8
```

Candid:

```python
service: {
    "get_nat8": () -> (nat8) query;
    "print_nat8": (nat8) -> (nat8) query;
}
```

#### int

The Kybra type `int` corresponds to the [Candid type int](https://internetcomputer.org/docs/current/references/candid-ref#type-int) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int, query

@query
def get_int() -> int:
    return 170_141_183_460_469_231_731_687_303_715_884_105_727

@query
def print_int(int: int) -> int:
    ic.print(type(int))
    return int
```

Candid:

```python
service: {
    "get_int": () -> (int) query;
    "print_int": (int) -> (int) query;
}
```

#### int64

The Kybra type `int64` corresponds to the [Candid type int64](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, inat64, query

@query
def get_int64() -> int64:
    return 9_223_372_036_854_775_807

@query
def print_int64(int64: int64) -> int64:
    ic.print(type(int64))
    return int64
```

Candid:

```python
service: {
    "get_int64": () -> (int64) query;
    "print_int64": (int64) -> (int64) query;
}
```

#### int32

The Kybra type `int32` corresponds to the [Candid type int32](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int32, query

@query
def get_int32() -> int32:
    return 2_147_483_647

@query
def print_int32(int32: int32) -> int32:
    ic.print(type(int32))
    return int32
```

Candid:

```python
service: {
    "get_int32": () -> (int32) query;
    "print_int32": (int32) -> (int32) query;
}
```

#### int16

The Kybra type `int16` corresponds to the [Candid type int16](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int16, query

@query
def get_int16() -> int16:
    return 32_767

@query
def print_int16(int16: int16) -> int16:
    ic.print(type(int16))
    return int16
```

Candid:

```python
service: {
    "get_int16": () -> (int16) query;
    "print_int16": (int16) -> (int16) query;
}
```

#### int8

The Kybra type `int8` corresponds to the [Candid type int8](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int8, query

@query
def get_int8() -> int8:
    return 127

@query
def print_int8(int8: int8) -> int8:
    ic.print(type(int8))
    return int8
```

Candid:

```python
service: {
    "get_int8": () -> (int8) query;
    "print_int8": (int8) -> (int8) query;
}
```

#### float64

The Kybra type `float64` corresponds to the [Candid type float64](https://internetcomputer.org/docs/current/references/candid-ref#type-float32-and-float64) and will become a [Python float](https://docs.python.org/3/library/functions.html#float) at runtime.

Python:

```python
from kybra import float64, ic, query

import math

@query
def get_float64() -> float64:
    return math.e

@query
def print_float64(float64: float64) -> float64:
    ic.print(type(float64))
    return float64
```

Candid:

```python
service: {
    "get_float64": () -> (float64) query;
    "print_float64": (float64) -> (float64) query;
}
```

#### float32

The Kybra type `float32` corresponds to the [Candid type float32](https://internetcomputer.org/docs/current/references/candid-ref#type-float32-and-float64) and will become a [Python float](https://docs.python.org/3/library/functions.html#float) at runtime.

Python:

```python
from kybra import float32, ic, query

def get_float32() -> float32:
    return math.pi

def print_float32(float32: float32) -> float32:
    ic.print(type(float32))
    return float32
```

Candid:

```python
service: {
    "get_float32": () -> (float32) query;
    "print_float32": (float32) -> (float32) query;
}
```

#### bool

The Python type `bool` corresponds to the [Candid type bool](https://internetcomputer.org/docs/current/references/candid-ref#type-bool) and will become a [Python Boolean Value](https://docs.python.org/3/library/stdtypes.html#boolean-values) at runtime.

Python:

```Python
from kybra import ic, query

@query
def get_bool() -> bool:
    return True

@query
def print_bool(bool: bool) -> bool:
    ic.print(type(bool))
    return bool
```

Candid:

```python
service: {
    "get_bool": () -> (bool) query;
    "print_bool": (bool) -> (bool) query;
}
```

#### null

The Python type `None` and the Kybra type `null` both correspond to the [Candid type null](https://internetcomputer.org/docs/current/references/candid-ref#type-null) and will become the [Python Null Object](https://docs.python.org/3/library/stdtypes.html#the-null-object) at runtime.

Python:

```python
from kybra import ic, query

@query
def get_null() -> None:
    return None

@query
def print_null(none: None) -> None:
    ic.print(type(none))
    return none
```

Candid:

```python
service: {
    "get_null": () -> (null) query;
    "print_null": (null) -> (null) query;
}
```

#### vec

The Python type `list` corresponds to the [Candid type vec](https://internetcomputer.org/docs/current/references/candid-ref#type-vec-t) and will become an array of the specified type at runtime.

Python:

```python
from kybra import int32, query

@query
def get_numbers() -> list[int32]:
    return [0, 1, 2, 3]
```

Candid:

```python
service: {
    "get_numbers": () -> (vec int32) query;
}
```

#### opt

The Kybra type `opt` corresponds to the [Candid type opt](https://internetcomputer.org/docs/current/references/candid-ref#type-opt-t) and will become the enclosed Python type or None at runtime.

Python:

```python
from kybra import opt, query

@query
def get_opt_some() -> opt[bool]:
    return True

@query
def get_opt_none() -> opt[bool]:
    return None
```

Candid:

```python
service: {
    "get_opt_some": () -> (opt bool) query;
    "get_opt_none": () -> (opt bool) query;
}
```

#### record

Python classes that inherit from the Kybra type `Record` correspond to the [Candid record type](https://internetcomputer.org/docs/current/references/candid-ref#type-record--n--t--) and will become [Python TypedDicts](https://docs.python.org/3/library/typing.html#typing.TypedDict) at runtime.

Python:

```python
from kybra import Record

class Post(Record):
    id: str
    author: 'User'
    text: str
    thread: 'Thread'

class Thread(Record):
    id: str
    author: 'User'
    posts: list[Post]
    title: str

class User(Record):
    id: str
    posts: list[Post]
    thread: list[Thread]
    username: str
```

Candid:

```python
type Post = record {
    "id": text;
    "author": User;
    "text": text;
    "thread": Thread;
};

type Thread = record {
    "id": text;
    "author": User;
    "posts": vec Post;
    "title": text;
};

type User = record {
    "id": text;
    "posts": vec Post;
    "threads": vec Thread;
    "username": text;
};
```

#### variant

Python classes that inherit from the Kybra type `Variant` correspond to the [Candid variant type](https://internetcomputer.org/docs/current/references/candid-ref#type-variant--n--t--) and will become [Python TypedDicts](https://docs.python.org/3/library/typing.html#typing.TypedDict) at runtime.

Python:

```python
from kybra import nat32, Variant

class ReactionType(Variant, total=False):
    Fire: None
    ThumbsUp: None
    ThumbsDown: None
    Emotion: 'Emotion'
    Firework: 'Firework'

class Emotion(Variant, total=False):
    Happy: None
    Sad: None

class Firework(Variant, total=False):
    Color: str
    NumStreaks: nat32
```

Candid:

```python
type ReactionType = variant {
    "Fire": null;
    "ThumbsUp": null;
    "ThumbsDown": null;
    "Emotion": Emotion;
    "Firework": Firework
};

type Emotion = variant {
    "Happy": null;
    "Sad": null
};

type Firework = record {
    "Color": text;
    "NumStreaks": nat32;
};
```

#### func

The Kybra type `Func` corresponds to the [Candid type func](https://internetcomputer.org/docs/current/references/candid-ref#type-func---) and at runtime will become a Python tuple with two elements, the first being an [ic-py Principal](https://github.com/rocklabs-io/ic-py) and the second being a [Python str](https://docs.python.org/3/library/stdtypes.html#textseq). The `ic-py Principal` represents the `principal` of the canister/service where the function exists, and the `str` represents the function's name.

Note that an explicit `TypeAlias` must be used when defining a `func`.

Python:

```python
from kybra import Func, nat64, Principal, query, Query, Record, update, Update, Variant

class User(Record):
    id: str
    basic_func: 'BasicFunc'
    complex_func: 'ComplexFunc'

class Reaction(Variant, total=False):
    Good: None
    Bad: None
    BasicFunc: 'BasicFunc'
    ComplexFunc: 'ComplexFunc'

BasicFunc: TypeAlias = Func(Query[[str], str])
ComplexFunc: TypeAlias = Func(Update[[User, Reaction], nat64])

@query
def get_basic_func() -> BasicFunc:
    return [
        Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai'),
        'simple_function_name'
    ]

@query
def get_complex_func() -> ComplexFunc:
    return [
        Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'),
        'complex_function_name'
    ]
```

Candid:

```python
type User = record {
    "id": text;
    "basic_func": BasicFunc;
    "complex_func": ComplexFunc;
};
type Reaction = variant { "Good": null; "Bad": null; "BasicFunc": BasicFunc; "ComplexFunc": ComplexFunc };

type BasicFunc = func (text) -> (text) query;
type ComplexFunc = func (User, Reaction) -> (nat64);

service: () -> {
    "get_basic_func": () -> (BasicFunc) query;
    "get_complex_func": () -> (ComplexFunc) query;
}

```

#### service

[Not yet implemented.](https://github.com/demergent-labs/kybra/issues/124)

#### principal

The Kybra type `Principal` corresponds to the [Candid type principal](https://internetcomputer.org/docs/current/references/candid-ref#type-principal) and will become an [ic-py Principal](https://github.com/rocklabs-io/ic-py) at runtime.

Python:

```python
from kybra import ic, Principal, query

@query
def get_principal() -> Principal:
    return Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai')

@query
def print_principal(principal: Principal) -> Principal:
    ic.print(type(principal))
    return principal
```

Candid:

```python
service: {
    "get_principal": () -> (principal) query;
    "print_principal": (principal) -> (principal) query;
}
```

#### reserved

The Kybra type `reserved` corresponds to the [Candid type reserved](https://internetcomputer.org/docs/current/references/candid-ref#type-reserved) and will become the [Python Null Object](https://docs.python.org/3/library/stdtypes.html#the-null-object) at runtime.

Python:

```python
from kybra import ic, query, reserved

@query
def get_reserved() -> reserved:
    return 'anything'

@query
def print_reserved(reserved: reserved) -> reserved:
    ic.print(type(reserved))
    return reserved
```

Candid:

```python
service: {
    "get_reserved": () -> (reserved) query;
    "print_reserved": (reserved) -> (reserved) query;
}
```

#### empty

The Kybra type `empty` corresponds to the [Candid type empty](https://internetcomputer.org/docs/current/references/candid-ref#type-empty) and has no Python value at runtime.

Python:

```python
from kybra import empty, ic, query

def get_empty() -> empty:
    raise Exception('Anything you want')

# Note: It is impossible to call this function because it requires an argument
# but there is no way to pass an "empty" value as an argument.
@query
def print_empty(empty: empty) -> empty:
    ic.print(type(empty))
    raise Exception('Anything you want')
```

Candid:

```python
service: {
    "get_empty": () -> (empty) query;
    "print_empty": (empty) -> (empty) query;
}
```

### Canister APIs

-   [Candid decode](#candid-decode)
-   [Candid encode](#candid-encode)
-   [canister balance](#canister-balance)
-   [canister balance 128](#canister-balance-128)
-   [data certificate](#data-certificate)
-   [canister id](#canister-id)
-   [print](#print)
-   [set certified data](#set-certified-data)
-   [time](#time)
-   [trap](#trap)

#### Candid decode

Examples:

-   [call_raw](/examples/call_raw)
-   [candid_encoding](/examples/candid_encoding)

```python
from kybra import blob, ic, query

@query
def candid_decode(candid_encoded: blob) -> str:
    return ic.candid_decode(candid_encoded)
```

#### Candid encode

Examples:

-   [candid_encoding](/examples/candid_encoding)
-   [manual_reply](/examples/manual_reply)
-   [notify_raw](/examples/notify_raw)

```python
from kybra import blob, ic, query

# encodes a Candid string to Candid bytes
@query
def candid_encode(candid_string: str) -> blob:
    return ic.candid_encode(candid_string)
```

#### canister balance

Examples:

-   [cycles](/examples/cycles)
-   [ic_api](/examples/ic_api)

```python
from kybra import ic, nat64, query

# returns the amount of cycles available in the canister
@query
def canister_balance() -> nat64:
    return ic.canister_balance()
```

#### canister balance 128

Examples:

-   [cycles](/examples/cycles)
-   [ic_api](/examples/ic_api)

```python
from kybra import ic, nat, query

# returns the amount of cycles available in the canister
@query
def canister_balance128() -> nat:
    return ic.canister_balance128()
```

#### data certificate

Examples:

-   [ic_api](/examples/ic_api)

```python
from kybra import blob, ic, opt, query

# When called from a query call, returns the data certificate authenticating certified_data set by this canister. Returns None if called not from a query call.
@query
def data_certificate() -> opt[blob]:
    return ic.data_certificate()
```

#### canister id

Examples:

-   [http_counter](/examples/motoko_examples/http_counter)
-   [ic_api](/examples/ic_api)
-   [whoami](/examples/motoko_examples/whoami)

```python
from kybra import ic, Principal, query

# returns this canister's id
@query
def id() -> Principal:
    return ic.id()
```

#### print

Examples:

-   [ic_api](/examples/ic_api)

```python
from kybra import ic, query

# prints a message through the local replica's output
@query
def print(message: str) -> bool:
    ic.print(message)

    return true
```

#### set certified data

Examples:

-   [ic_api](/examples/ic_api)

```python
from kybra import blob, ic, update

# sets up to 32 bytes of certified data
@update
def set_certified_data(data: blob):
    ic.set_certified_data(data)
```

#### time

Examples:

-   [ic_api](/examples/ic_api)

```python
from kybra import ic, nat64, query

# returns the current timestamp
@query
def time() -> nat64:
    return ic.time()
```

#### trap

Examples:

-   [cross_canister_calls](/examples/cross_canister_calls)
-   [http_counter](/examples/motoko_examples/http_counter)
-   [ic_api](/examples/ic_api)

```python
from kybra import ic, query

# traps with a message, stopping execution and discarding all state within the call
@query
def trap(message: str) -> bool:
    ic.trap(message)

    return true
```

### Call APIs

-   [caller](#caller)
-   [accept message](#accept-message)
-   [arg data](#arg-data)
-   [arg data raw](#arg-data-raw)
-   [arg data raw size](#arg-data-raw-size)
-   [call](#call)
-   [call raw](#call-raw)
-   [call raw 128](#call-raw-128)
-   [call with payment](#call-with-payment)
-   [call with payment 128](#call-with-payment-128)
-   [method name](#method-name)
-   [msg cycles accept](#msg-cycles-accept)
-   [msg cycles accept 128](#msg-cycles-accept-128)
-   [msg cycles available](#msg-cycles-available)
-   [msg cycles available 128](#msg-cycles-available-128)
-   [msg cycles refunded](#msg-cycles-refunded)
-   [msg cycles refunded 128](#msg-cycles-refunded-128)
-   [notify](#notify)
-   [notify raw](#notify-raw)
-   [notify with payment 128](#notify-with-payment-128)
-   [performance counter](#performance-counter)
-   [reject](#reject)
-   [reject code](#reject-code)
-   [reject message](#reject-message)
-   [reply](#reply)
-   [reply raw](#reply-raw)
-   [result](#result)

#### caller

Examples:

-   [ic_api](/examples/ic_api)
-   [whoami](/examples/motoko_examples/whoami)

```python
from kybra import ic, Principal, query

# returns the principal of the identity that called this function
@query
def caller() -> Principal:
    return ic.caller()
```

#### accept message

Examples:

-   [inspect_message](/examples/inspect_message)

```python
from kybra import ic, inspect_message, update

@inspect_message
def inspect_message_():
    ic.print('this runs before executing update calls')

    if ic.method_name() == 'accessible':
        ic.accept_message()
        return

    if ic.method_name() == 'inaccessible':
        return

    raise Exception(f'Method "{ic.method_name()}" not allowed')

@update
def accessible() -> bool:
    return True

@update
def inaccessible() -> bool:
    return False

@update
def also_inaccessible() -> bool:
    return False
```

#### arg data

[Not yet implemented.](https://github.com/demergent-labs/azle/issues/496)

#### arg data raw

Examples:

-   [ic_api](/examples/ic_api)

```python
from kybra import blob, ic, int8, query

# returns the argument data as bytes.
@query
def arg_data_raw(
    arg1: blob,
    arg2: int8,
    arg3: bool,
    arg4: str
) -> blob:
    return ic.arg_data_raw()
```

#### arg data raw size

Examples:

-   [ic_api](/examples/ic_api)

```python
from kybra import blob, ic, int8, nat32, query

# returns the length of the argument data in bytes
@query
def arg_data_raw_size(
    arg1: blob,
    arg2: int8,
    arg3: bool,
    arg4: str
) -> nat32:
    return ic.arg_data_raw_size()
```

#### call

Examples:

-   [cross_canister_calls](/examples/cross_canister_calls)
-   [cycles](/examples/cycles)
-   [func_types](/examples/func_types)
-   [rejections](/examples/rejections)
-   [tuple_types](/examples/tuple_types)
-   [whoami](/examples/motoko_examples/whoami)

```python
from kybra import Async, Canister, CanisterResult, ic, method, Principal, update, Variant

class Canister1(Canister):
    @method
    def example_method(self) -> bool: ...

canister1 = Canister1(Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai'))

class CallCanister1ExampleMethodResult(Variant, total=False):
    ok: bool
    err: str

@update
def call_canister1_example_method() -> Async[CallCanister1ExampleMethodResult]:
    canister_result: CanisterResult[bool] = yield canister1.example_method()

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': canister_result.ok
    }
```

#### call raw

Examples:

-   [call_raw](/examples/call_raw)

```python
from kybra import Async, blob, CanisterResult, ic, Principal, update

@update
def get_randomness() -> Async[blob]:
    canister_result: CanisterResult[blob] = yield ic.call_raw(
        Principal.from_str('aaaaa-aa'),
        'raw_rand',
        ic.candid_encode('()'),
        0 # this is a nat64
    )

    if canister_result.err is not None:
        return bytes()

    return canister_result.ok
```

#### call raw 128

Examples:

-   [call_raw](/examples/call_raw)

```python
from kybra import Async, blob, CanisterResult, ic, Principal, update

@update
def get_randomness() -> Async[blob]:
    canister_result: CanisterResult[blob] = yield ic.call_raw128(
        Principal.from_str('aaaaa-aa'),
        'raw_rand',
        ic.candid_encode('()'),
        0 # this is a nat
    )

    if canister_result.err is not None:
        return bytes()

    return canister_result.ok
```

#### call with payment

Examples:

-   [cycles](/examples/cycles)
-   [management_canister](/examples/management_canister)

```python
from kybra import Async, Canister, CanisterResult, ic, method, Principal, update, Variant

class Canister1(Canister):
    @method
    def example_method(self) -> bool: ...

canister1 = Canister1(
    Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai')
)

class CallCanister1MethodResult(Variant, total=False):
    ok: bool
    err: str

@update
def call_canister1_method() -> Async[CallCanister1MethodResult]:
    canister_result: CanisterResult[bool] = yield canister1.example_method().with_cycles(100_000_000_000)

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': canister_result.ok
    }
```

#### call with payment 128

Examples:

-   [cycles](/examples/cycles)

```python
from kybra import Async, Canister, CanisterResult, ic, method, Principal, update, Variant

class Canister1(Canister):
    @method
    def example_method(self) -> bool: ...

canister1 = Canister1(
    Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai')
)

class CallCanister1MethodResult(Variant, total=False):
    ok: bool
    err: str

@update
def call_canister1_method() -> Async[CallCanister1MethodResult]:
    canister_result: CanisterResult[bool] = yield canister1.example_method().with_cycles128(100_000_000_000)

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': canister_result.ok
    }
```

#### method name

Examples:

-   [inspect_message](/examples/inspect_message)

```python
from kybra import ic, inspect_message, update

@inspect_message
def inspect_message_():
    ic.print('this runs before executing update calls')

    if ic.method_name() === 'accessible':
        ic.accept_message()
        return

    if ic.method_name() === 'inaccessible':
        return

    raise Exception(f'Method "{ic.method_name()}" not allowed')

@update
def accessible() -> bool:
    return True

@update
def inaccessible() -> bool:
    return False

@update
def also_inaccessible() -> bool:
    return False
```

#### msg cycles accept

Examples:

-   [cycles](/examples/cycles)

```python
from kybra import ic, nat64, update

# Moves all transferred cycles to the canister
@update
def receive_cycles() -> nat64:
    return ic.msg_cycles_accept(ic.msg_cycles_available())
```

#### msg cycles accept 128

Examples:

-   [cycles](/examples/cycles)

```python
from kybra import ic, nat, update

# Moves all transferred cycles to the canister
@update
def receive_cycles128() -> nat:
    return ic.msg_cycles_accept128(ic.msg_cycles_available128())
```

#### msg cycles available

Examples:

-   [cycles](/examples/cycles)

```python
from kybra import ic, nat64, update

# Moves all transferred cycles to the canister
@update
def receive_cycles() -> nat64:
    return ic.msg_cycles_accept(ic.msg_cycles_available())
```

#### msg cycles available 128

Examples:

-   [cycles](/examples/cycles)

```python
from kybra import ic, nat64, update

# Moves all transferred cycles to the canister
@update
def receive_cycles128() -> nat64:
    return ic.msg_cycles_accept128(ic.msg_cycles_available128())
```

#### msg cycles refunded

Examples:

-   [cycles](/examples/cycles)

```python
from kybra import Canister, CanisterResult, ic, method, nat64, Principal, update, Variant

class Canister1(Canister):
    @method
    def example_method(self) -> bool: ...

canister1 = Canister1(
    Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai')
)

class CallCanister1MethodResult(Variant, total=False):
    ok: nat64
    err: str

@update
def call_canister1_method() -> CallCanister1MethodResult:
    canister_result: CanisterResult[bool] = yield canister1.example_method().with_cycles(100_000_000_000)

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': ic.msg_cycles_refunded()
    }
```

#### msg cycles refunded 128

Examples:

-   [cycles](/examples/cycles)

```python
from kybra import Async, Canister, CanisterResult, ic, nat, Principal, update, Variant

class Canister1(Canister):
    @method
    def example_method(self): bool: ...

canister1 = Canister1(
    Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai')
)

class CallCanister1MethodResult(Variant, total=False):
    ok: nat
    err: str

@update
def call_canister1_method() -> Async[CallCanister1MethodResult]:
    canister_result: CanisterResult[bool] = yield canister1.example_method().with_cycles128(100_000_000_000)

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': ic.msg_cycles_refunded128()
    }
```

#### notify

Examples:

-   [cross_canister_calls](/examples/cross_canister_calls)
-   [cycles](/examples/cycles)

```python
from kybra import Canister, CanisterResult, ic, method, Principal, Update

class Canister1(Canister):
    @method
    def example_method(self) -> bool: ...

canister1 = Canister1(
    Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai')
)

def call_canister1_method() -> bool:
    canister_result: CanisterResult[None] = canister1.example_method().notify()

    if 'err' in canister_result:
        return False

    return True
```

#### notify raw

Examples:

-   [notify_raw](/examples/notify_raw)

```python
from kybra import ic, Principal, update

@update
def send_notification() -> bool:
    result = ic.notify_raw(
        Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'),
        'receive_notification',
        ic.candid_encode('()'),
        0
    )

    if 'err' in result:
        return False

    return True
```

#### notify with payment 128

Examples:

-   [cycles](/examples/cycles)

```python
from kybra import Async, Canister, CanisterResult, ic, method, Principal, update

class Canister1(Canister):
    @method
    def example_method(self) -> bool: ...

canister1 = Canister1(
    Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai')
)

@update
def call_canister1_method() -> Async[bool]:
    canister_result: CanisterResult[None] = canister1.example_method().with_cycles128(100_000_000_000).notify()

    if canister_result.err is not None:
        return False

    return True
```

#### performance counter

Examples:

-   [ic_api](/examples/ic_api)

```python
from kybra import ic, nat64, query

@query
def performance_counter() -> nat64:
    return ic.performance_counter(0)
```

#### reject

Examples:

-   [ic_api](/examples/ic_api)
-   [manual_reply](/examples/manual_reply)
-   [rejections](/examples/rejections)

```python
from kybra import empty, ic, manual

def reject(message: str) -> manual[empty]:
    ic.reject(message)
```

#### reject code

Examples:

-   [rejections](/examples/rejections)

```python
from kybra import Async, Canister, CanisterResult, ic, Principal, RejectionCode, update

class Canister1(Canister):
    @method
    def example_method(self) -> bool: ...

canister1 = Canister1(
    Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai')
)

@update
def get_rejection_code() -> Async[RejectionCode]:
    yield canister1.example_method()
    return ic.reject_code()
```

#### reject message

Examples:

-   [rejections](/examples/rejections)

```python
from kybra import Async, Canister, CanisterResult, ic, method, Principal, update

class Canister1(Canister):
    @method
    def example_method(self) -> bool: ...

canister1 = Canister1(
    Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai')
)

@update
def get_rejection_message() -> Async[str]:
    yield canister1.example_method()
    return ic.reject_message()
```

#### reply

Examples:

-   [manual_reply](/examples/manual_reply)

```python
from kybra import ic, manual, update

def manual_update(message: str) -> manual[str]:
    if message == 'reject':
        ic.reject(message)
        return

    ic.reply(message)
```

#### reply raw

Examples:

-   [manual_reply](/examples/manual_reply)

```python
from kybra import blob, ic, int, manual, Record, Variant

class RawReply(Record):
    int: int
    text: str
    bool: bool
    blob: blob
    variant: 'Options'

class Options(Variant, total=False):
    Small: None
    Medium: None
    Large: None

@update
def reply_raw() -> manual[RawReply]:
    ic.reply_raw(
        ic.candid_encode(
            '(record { "int" = 42; "text" = "text"; "bool" = true; "blob" = blob "Surprise!"; "variant" = variant { Medium } })'
        )
    )
```

#### result

[Not yet implemented.](https://github.com/demergent-labs/azle/issues/496)

### Stable Memory

-   [stable storage](#stable-storage)
-   [stable64 grow](#stable64-grow)
-   [stable64 read](#stable64-read)
-   [stable64 size](#stable64-size)
-   [stable64 write](#stable64-write)
-   [stable bytes](#stable-bytes)
-   [stable grow](#stable-grow)
-   [stable read](#stable-read)
-   [stable size](#stable-size)
-   [stable write](#stable-write)

#### stable storage

The current stable storage implementation is limited in how much data can be serialized/deserialized in the pre_upgrade/post_upgrade step before the cycle limit is reached. It's unclear exactly what the serialization/deserialization limits are, but consider that they will most likely be significantly less than 4GiB.

This applies to stable storage only, not the more primitive stable memory operations.

To resolve these issues, we plan to release a Kybra-specific stable structure similar to what can be found [here](https://github.com/dfinity/stable-structures).

Examples:

-   [func_types](/examples/func_types)
-   [http_counter](/examples/motoko_examples/http_counter)
-   [persistent_storage](/examples/motoko_examples/persistent-storage)
-   [pre_and_post_upgrade](/examples/pre_and_post_upgrade)
-   [stable_storage](/examples/stable_storage)
-   [tuple_types](/examples/tuple_types)

```python
from kybra import init, nat, update

from typing import TypedDict

class StableStorage(TypedDict):
    stable_nat: nat

stable_storage: StableStorage = ic.stable_storage()

@init
def init_(stable_nat: nat):
    stable_storage['stable_nat'] = stable_nat
```

#### stable64 grow

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import ic, nat64, Stable64GrowResult, update

@update
def stable64_grow(new_pages: nat64) -> Stable64GrowResult:
    return ic.stable64_grow(new_pages)
```

#### stable64 read

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import blob, ic, nat64, query

@query
def stable64_read(offset: nat64, length: nat64) -> blob:
    return ic.stable64_read(offset, length)
```

#### stable64 size

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import ic, nat64, query

@query
def stable64_size() -> nat64:
    return ic.stable64_size()
```

#### stable64 write

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import blob, ic, nat64, update

@update
def stable64_write(offset: nat64, buf: blob):
    ic.stable64_write(offset, buf)
```

#### stable bytes

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import blob, ic, query

@query
def stable_bytes() -> blob:
    return ic.stable_bytes()
```

#### stable grow

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import ic, nat32, StableGrowResult, update

@update
def stable_grow(new_pages: nat32) -> StableGrowResult:
    return ic.stable_grow(new_pages)
```

#### stable read

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import blob, ic, nat32, query

@query
def stable_read(offset: nat32, length: nat32) -> blob:
    return ic.stable_read(offset, length)
```

#### stable size

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import ic, nat32, query

@query
def stable_size() -> nat32:
    return ic.stable_size()
```

#### stable write

Examples:

-   [stable_memory](/examples/stable_memory)

```python
from kybra import blob, ic, nat32, update

@update
def stable_write(offset: nat32, buf: blob):
    ic.stable_write(offset, buf)
```

### Python stdlib

There is limited support for the `stdlib`. The following modules may be supported as far as [RustPython](https://github.com/RustPython/RustPython) or the IC support them:

-   array
-   binascii
-   \_bisect
-   cmath
-   \_contextvars
-   \_csv
-   \_dis
-   gc
-   hashlib
-   \_json
-   math
-   pyexpat
-   \_struct
-   \_random (May not work yet: https://github.com/demergent-labs/kybra/issues/169)
-   \_statistics
-   unicodedata
-   zlib

### Python External Packages

Installing external packages, such as from [PyPI](https://pypi.org/), will probably not work. The biggest problem you are most likely to run into is lack of support for most of the Python stdlib. Once the Wasm binary limit is increased on the IC, much more of the stdlib should be supported which should open up many external packages.

Once the majority of the stdlib is supported, you are likely to run into issues with the environment that the Python code is running in. It is a Rust `wasm32-unknown-unknown` environment that has access to the IC APIs. Any external packages that can't compile or run in this environment with the supported APIs will not work.

Much of the future work for enabling external packages will be forking and patching packages to support IC APIs.

### Security

Things to keep in mind:

-   Kybra does not yet have many live, successful, continuously operating applications deployed to the IC
-   Kybra does not yet have extensive automated property tests
-   Kybra does not yet have multiple independent security reviews/audits

### Roadmap

The following are the major blockers to 1.0/production-readiness:

-   Majority stdlib support (~Q4 2022/Q1 2023)
-   Extensive automated property testing (~Q1 2023)
-   Multiple independent security reviews/audits (~Q1/Q2 2023)
-   Performance improvements if necessary (~2023)

### Gotchas and Caveats

-   Most of the stdlib doesn't work yet
-   Most PyPI packages will not work yet

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
