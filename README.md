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

## Security

Things to keep in mind:

-   Kybra does not yet have many live, successful, continuously operating applications deployed to the IC
-   Kybra does not yet have extensive automated property tests
-   Kybra does not yet have multiple independent security reviews/audits
-   Kybra heavily relies on RustPython which is [self-proclaimed to not be totally production-ready](https://github.com/RustPython/RustPython#disclaimer)

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
-   [Special APIs](#special-apis)
-   [JS APIs](#js-apis)
-   [Feature Parity](#feature-parity)
-   [Security](#security)
-   [Benchmarks](#benchmarks)
-   [Roadmap](#roadmap)
-   [Gotchas and Caveats](#gotchas-and-caveats)
-   [Decentralization](#decentralization)
-   [Contributing](#contributing)
-   [License](#license)

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

-   You may need to manually install `ic-cdk-optimizer`: `cargo install ic-cdk-optimizer`
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

Users of your canister interact with it through RPC calls performed using HTTP requests. These calls will hit your canister's `Query` and `Update` methods. These methods, with their parameter and return types, are the interface to your canister.

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

-   [basic-dao](/examples/motoko_examples/basic-dao)
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

-   [basic-dao](/examples/motoko_examples/basic-dao)
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

-   [basic-dao](/examples/motoko_examples/basic-dao)
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

-   [basic-dao](/examples/motoko_examples/basic-dao)
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

Update calls go through consensus and thus return very slowly (a few seconds) relative to query calls. This also means they are more secure than query calls unless [certified data](https://smartcontracts.org/docs/base-libraries/certifieddata) is used in conjunction with the query call.

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

Query calls do not go through consensus and thus return very quickly relative to update calls. This also means they are less secure than update calls unless [certified data](https://smartcontracts.org/docs/base-libraries/certifieddata) is used in conjunction with the query call.

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

The Python type `str` corresponds to the [Candid type text](https://internetcomputer.org/docs/current/references/candid-ref#type-text) and will become a [Python str](https://docs.python.org/3/library/stdtypes.html#textseq) at runtime.

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

The Kybra type `blob` corresponds to the [Candid type blob](https://internetcomputer.org/docs/current/references/candid-ref#type-blob) and will become a [JavaScript Uint8Array](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) at runtime.

TypeScript:

```typescript
import { blob, Query } from 'azle';

export function get_blob(): Query<blob> {
    return Uint8Array.from([68, 73, 68, 76, 0, 0]);
}

export function print_blob(blob: blob): Query<blob> {
    console.log(typeof blob);
    return blob;
}
```

Candid:

```typescript
service: {
    "get_blob": () -> (blob) query;
    "print_blob": (blob) -> (blob) query;
}
```

#### nat

The Azle type `nat` corresponds to the [Candid type nat](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-nat) and will become a [JavaScript BigInt](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt) at runtime.

TypeScript:

```typescript
import { nat, Query } from 'azle';

export function get_nat(): Query<nat> {
    return 340_282_366_920_938_463_463_374_607_431_768_211_455n;
}

export function print_nat(nat: nat): Query<nat> {
    console.log(typeof nat);
    return nat;
}
```

Candid:

```typescript
service: {
    "get_nat": () -> (nat) query;
    "print_nat": (nat) -> (nat) query;
}
```

#### nat64

The Azle type `nat64` corresponds to the [Candid type nat64](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-intN) and will become a [JavaScript BigInt](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt) at runtime.

TypeScript:

```typescript
import { nat64, Query } from 'azle';

export function get_nat64(): Query<nat64> {
    return 18_446_744_073_709_551_615n;
}

export function print_nat64(nat64: nat64): Query<nat64> {
    console.log(typeof nat64);
    return nat64;
}
```

Candid:

```typescript
service: {
    "get_nat64": () -> (nat64) query;
    "print_nat64": (nat64) -> (nat64) query;
}
```

#### nat32

The Azle type `nat32` corresponds to the [Candid type nat32](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-intN) and will become a [JavaScript Number](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number) at runtime.

TypeScript:

```typescript
import { nat32, Query } from 'azle';

export function get_nat32(): Query<nat32> {
    return 4_294_967_295;
}

export function print_nat32(nat32: nat32): Query<nat32> {
    console.log(typeof nat32);
    return nat32;
}
```

Candid:

```typescript
service: {
    "get_nat32": () -> (nat32) query;
    "print_nat32": (nat32) -> (nat32) query;
}
```

#### nat16

The Azle type `nat16` corresponds to the [Candid type nat16](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-intN) and will become a [JavaScript Number](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number) at runtime.

TypeScript:

```typescript
import { nat16, Query } from 'azle';

export function get_nat16(): Query<nat16> {
    return 65_535;
}

export function print_nat16(nat16: nat16): Query<nat16> {
    console.log(typeof nat16);
    return nat16;
}
```

Candid:

```typescript
service: {
    "get_nat16": () -> (nat16) query;
    "print_nat16": (nat16) -> (nat16) query;
}
```

#### nat8

The Azle type `nat8` corresponds to the [Candid type nat8](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-intN) and will become a [JavaScript Number](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number) at runtime.

TypeScript:

```typescript
import { nat8, Query } from 'azle';

export function get_nat8(): Query<nat8> {
    return 255;
}

export function print_nat8(nat8: nat8): Query<nat8> {
    console.log(typeof nat8);
    return nat8;
}
```

Candid:

```typescript
service: {
    "get_nat8": () -> (nat8) query;
    "print_nat8": (nat8) -> (nat8) query;
}
```

#### int

The Azle type `int` corresponds to the [Candid type int](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-int) and will become a [JavaScript BigInt](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt) at runtime.

TypeScript:

```typescript
import { int, Query } from 'azle';

export function get_int(): Query<int> {
    return 170_141_183_460_469_231_731_687_303_715_884_105_727n;
}

export function print_int(int: int): Query<int> {
    console.log(typeof int);
    return int;
}
```

Candid:

```typescript
service: {
    "get_int": () -> (int) query;
    "print_int": (int) -> (int) query;
}
```

#### int64

The Azle type `int64` corresponds to the [Candid type int64](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-intN) and will become a [JavaScript BigInt](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt) at runtime.

TypeScript:

```typescript
import { int64, Query } from 'azle';

export function get_int64(): Query<int64> {
    return 9_223_372_036_854_775_807n;
}

export function print_int64(int64: int64): Query<int64> {
    console.log(typeof int64);
    return int64;
}
```

Candid:

```typescript
service: {
    "get_int64": () -> (int64) query;
    "print_int64": (int64) -> (int64) query;
}
```

#### int32

The Azle type `int32` corresponds to the [Candid type int32](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-intN) and will become a [JavaScript Number](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number) at runtime.

TypeScript:

```typescript
import { int32, Query } from 'azle';

export function get_int32(): Query<int32> {
    return 2_147_483_647;
}

export function print_int32(int32: int32): Query<int32> {
    console.log(typeof int32);
    return int32;
}
```

Candid:

```typescript
service: {
    "get_int32": () -> (int32) query;
    "print_int32": (int32) -> (int32) query;
}
```

#### int16

The Azle type `int16` corresponds to the [Candid type int16](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-intN) and will become a [JavaScript Number](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number) at runtime.

TypeScript:

```typescript
import { int16, Query } from 'azle';

export function get_int16(): Query<int16> {
    return 32_767;
}

export function print_int16(int16: int16): Query<int16> {
    console.log(typeof int16);
    return int16;
}
```

Candid:

```typescript
service: {
    "get_int16": () -> (int16) query;
    "print_int16": (int16) -> (int16) query;
}
```

#### int8

The Azle type `int8` corresponds to the [Candid type int8](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-intN) and will become a [JavaScript Number](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number) at runtime.

TypeScript:

```typescript
import { int8, Query } from 'azle';

export function get_int8(): Query<int8> {
    return 127;
}

export function print_int8(int8: int8): Query<int8> {
    console.log(typeof int8);
    return int8;
}
```

Candid:

```typescript
service: {
    "get_int8": () -> (int8) query;
    "print_int8": (int8) -> (int8) query;
}
```

#### float64

The Azle type `float64` corresponds to the [Candid type float64](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-floatN) and will become a [JavaScript Number](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number) at runtime.

TypeScript:

```typescript
import { float64, Query } from 'azle';

export function get_float64(): Query<float64> {
    return Math.E;
}

export function print_float64(float64: float64): Query<float64> {
    console.log(typeof float64);
    return float64;
}
```

Candid:

```typescript
service: {
    "get_float64": () -> (float64) query;
    "print_float64": (float64) -> (float64) query;
}
```

#### float32

The Azle type `float32` corresponds to the [Candid type float32](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-floatN) and will become a [JavaScript Number](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number) at runtime.

TypeScript:

```typescript
import { float32, Query } from 'azle';

export function get_float32(): Query<float32> {
    return Math.PI;
}

export function print_float32(float32: float32): Query<float32> {
    console.log(typeof float32);
    return float32;
}
```

Candid:

```typescript
service: {
    "get_float32": () -> (float32) query;
    "print_float32": (float32) -> (float32) query;
}
```

#### bool

The TypeScript type `boolean` corresponds to the [Candid type bool](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-bool) and will become a [JavaScript Boolean](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean) at runtime.

TypeScript:

```typescript
import { Query } from 'azle';

export function get_bool(): Query<boolean> {
    return true;
}

export function print_bool(bool: boolean): Query<boolean> {
    console.log(typeof bool);
    return bool;
}
```

Candid:

```typescript
service: {
    "get_bool": () -> (bool) query;
    "print_bool": (bool) -> (bool) query;
}
```

#### null

The TypeScript type `null` corresponds to the [Candid type null](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-null) and will become a [JavaScript null](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/null) at runtime.

TypeScript:

```typescript
import { Query } from 'azle';

export function get_null(): Query<null> {
    return null;
}

export function print_null(_null: null): Query<null> {
    console.log(typeof _null);
    return _null;
}
```

Candid:

```typescript
service: {
    "get_null": () -> (null) query;
    "print_null": (null) -> (null) query;
}
```

#### vec

TypeScript `[]` array syntax corresponds to the [Candid type vec](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-vec) and will become an array of the specified type at runtime (except for `nat8[]` which will become a `Uint8Array`, thus it is recommended to use the `blob` type instead of `nat8[]`). Only the `[]` array syntax is supported at this time (i.e. not `Array` or `ReadonlyArray` etc).

TypeScript:

```typescript
import { int32, Query } from 'azle';

export function get_numbers(): Query<int32[]> {
    return [0, 1, 2, 3];
}
```

Candid:

```typescript
service: {
    "get_numbers": () -> (vec int32) query;
}
```

#### opt

The Azle type `Opt` corresponds to the [Candid type opt](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-opt) and will become the enclosed JavaScript type or null at runtime.

TypeScript:

```typescript
import { Opt, Query } from 'azle';

export function get_opt_some(): Query<Opt<boolean>> {
    return true;
}

export function get_opt_none(): Query<Opt<boolean>> {
    return null;
}
```

Candid:

```typescript
service: {
    "get_opt_some": () -> (opt bool) query;
    "get_opt_none": () -> (opt bool) query;
}
```

#### record

TypeScript type aliases referring to object literals correspond to the [Candid record type](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-record) and will become [JavaScript Objects](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object) at runtime.

TypeScript:

```typescript
type Post = {
    id: string;
    author: User;
    text: string;
    thread: Thread;
};

type Thread = {
    id: string;
    author: User;
    posts: Post[];
    title: string;
};

type User = {
    id: string;
    posts: Post[];
    threads: Thread[];
    username: string;
};
```

Candid:

```typescript
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

TypeScript type aliases referring to object literals wrapped in the `Variant` Azle type correspond to the [Candid variant type](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-variant) and will become [JavaScript Objects](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object) at runtime.

TypeScript:

```typescript
import { nat32, Variant } from 'azle';

type ReactionType = Variant<{
    Fire: null;
    ThumbsUp: null;
    ThumbsDown: null;
    Emotion: Emotion;
    Firework: Firework;
}>;

type Emotion = Variant<{
    Happy: null;
    Sad: null;
}>;

type Firework = {
    Color: string;
    NumStreaks: nat32;
};
```

Candid:

```typescript
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

The Azle type `func` corresponds to the [Candid type func](https://internetcomputer.org/docs/current/references/candid-ref/#type-func---) and at runtime will become a JavaScript array with two elements, the first being an [@dfinity/principal](https://www.npmjs.com/package/@dfinity/principal) and the second being a [JavaScript string](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String). The `@dfinity/principal` represents the `principal` of the canister/service where the function exists, and the `string` represents the function's name.

TypeScript:

```typescript
import { Func, nat64, Principal, Query, Update, Variant } from 'azle';

type User = {
    id: string;
    basic_func: BasicFunc;
    complex_func: ComplexFunc;
};

type Reaction = Variant<{
    Good: null;
    Bad: null;
    BasicFunc: BasicFunc;
    ComplexFunc: ComplexFunc;
}>;

type BasicFunc = Func<(param1: string) => Query<string>>;
type ComplexFunc = Func<(user: User, reaction: Reaction) => Update<nat64>>;

export function get_basic_func(): Query<BasicFunc> {
    return [
        Principal.fromText('rrkah-fqaaa-aaaaa-aaaaq-cai'),
        'simple_function_name'
    ];
}

export function get_complex_func(): Query<ComplexFunc> {
    return [
        Principal.fromText('ryjl3-tyaaa-aaaaa-aaaba-cai'),
        'complex_function_name'
    ];
}
```

Candid:

```typescript
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

[Not yet implemented.](https://github.com/demergent-labs/azle/issues/445)

#### principal

The Azle type `Principal` corresponds to the [Candid type principal](https://smartcontracts.org/docs/candid-guide/candid-types.html#type-principal) and will become an [@dfinity/principal](https://www.npmjs.com/package/@dfinity/principal) at runtime.

TypeScript:

```typescript
import { Principal, Query } from 'azle';

export function get_principal(): Query<Principal> {
    return Principal.fromText('rrkah-fqaaa-aaaaa-aaaaq-cai');
}

export function print_principal(principal: Principal): Query<Principal> {
    console.log(typeof principal);
    return principal;
}
```

Candid:

```typescript
service: {
    "get_principal": () -> (principal) query;
    "print_principal": (principal) -> (principal) query;
}
```

-   Note that `Principal.selfAuthenticating` will not function properly until [this issue is resolved](https://github.com/boa-dev/boa/issues/1917)

#### reserved

The Azle type `reserved` corresponds to the [Candid type reserved](https://internetcomputer.org/docs/current/references/candid-ref/#type-reserved) and will become a [JavaScript null](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/null) at runtime.

TypeScript:

```typescript
import { Query, reserved } from 'azle';

export function get_reserved(): Query<reserved> {
    return 'anything';
}

export function print_reserved(reserved: reserved): Query<reserved> {
    console.log(typeof reserved);
    return reserved;
}
```

Candid:

```typescript
service: {
    "get_reserved": () -> (reserved) query;
    "print_reserved": (reserved) -> (reserved) query;
}
```

#### empty

The Azle type `empty` corresponds to the [Candid type empty](https://internetcomputer.org/docs/current/references/candid-ref/#type-empty) and has no JavaScript value at runtime.

TypeScript:

```typescript
import { empty, Query } from 'azle';

export function get_empty(): Query<empty> {
    throw 'Anything you want';
}

// Note: It is impossible to call this function because it requires an argument
// but there is no way to pass an "empty" value as an argument.
export function print_empty(empty: empty): Query<empty> {
    console.log(typeof empty);
    throw 'Anything you want';
}
```

Candid:

```typescript
service: {
    "get_empty": () -> (empty) query;
    "print_empty": (empty) -> (empty) query;
}
```

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
