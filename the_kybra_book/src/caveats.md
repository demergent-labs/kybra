# Caveats

## Unknown security vulnerabilities

Kybra is a beta project using a new Python interpreter. See the [disclaimer](./kybra.md#disclaimer) for more information.

## cdk metadata

-   All Kybra canisters have a public metadata key-value pair named `cdk` which contains the version of Kybra the canister is using
    -   Anyone can query this metadata which may be a security risk

## No C extensions

Any PyPI packages or other Python code that relies on C extensions will not currently work. It is a major goal for us to support C extensions in the future.

## Performance

Kybra is probably ~7-20x less performant than what you would expect from [CPython](https://github.com/python/cpython). We hope to eventually use `CPython` as Kybra's underlying Python interpreter.

## init and post_upgrade

### under-the-hood nuances

In order to allow for larger Kybra Wasm modules and to enable the Python stdlib, we had to introduce some trade-offs into the `init` and `post_upgrade` processes:

1. The under-the-hood Rust `init` function of your application canister is never called directly, only the `post_upgrade` function. This function will call your Python `init` or `post_upgrade` function in a timer callback
2. When deploying to a local replica, errors from your `init` or `post_upgrade` will be logged from your replica, not from the `dfx deploy` process
3. When deploying to mainnet, errors from your `init` or `post_upgrade` will not be logged
4. Your canister will go through the following main steps when deploying: first a deployer canister Wasm binary will be deployed to the canister, then your application Wasm binary will be chunk uploaded to the deployer canister along with the Python stdlib, and finally the deployer canister will call `install_code` on itself with the full application Wasm binary

### params

If you add parameters to your `init` or `post_upgrade` methods, you will need to manually add these parameters to the service in your Candid file for the first deploy to succeed. Any time you change these parameters, you will need to manually add these changes into your Candid file.

It is also recommended to start clean (`dfx start --clean`) when first deploying after changing these parameters.

Here's an example of a basic Candid file with parameters `text`, `bool`, and `int32`:

```
service : (text, bool, int32) -> {}
```

### guard functions

`init` and `post_upgrade` cannot have guard functions applied to them.

## No encrypted dfx identities

Kybra currently does not support encrypted identities. If you are asked for a password during `dfx deploy`, you'll need to create a new unencrypted dfx identity:

```bash
dfx identity new test_unencrypted --storage-mode plaintext
dfx identity use test_unencrypted

dfx deploy
```

## No candid:service metadata

Kybra does not put the Candid metadata into `candid:service` automatically. Currently if you want to retrieve the Candid from the canister you can call the `__get_candid_interface_tmp_hack` method.

## Wasm module hash now less useful

Due to protocol and tooling limitations, Kybra must dynamically upload your application's Python bytecode and the Python stdlib bytecode. This means that the Wasm module hash is not computed from all of your application's source code. Thus the Wasm module hash retrieved from the IC should not be relied upon for security assumptions.

## ic.caller() in init and post_upgrade

Usually `ic.caller()` if called from `init` or `post_upgrade` is the principal of your local dfx identity. Keep in mind that in Kybra canisters `ic.caller()` if called from `init` or `post_upgrade` is the canister's own principal.

## Do not use dictionary unpacking

A bug in the [RustPython](https://github.com/RustPython/RustPython) interpreter means that dictionary unpacking should not be used until [this issue](https://github.com/RustPython/RustPython/issues/4932) is addressed.

## Reserved memory ids

`memory ids` `0`, `1`, `2`, `252`, `253`, and `254` are currently reserved for `StableBTreeMap`.

## print does not work

You should use `ic.print` instead of `print`.

## Kybra types

### Imports

Make sure to use the `from kybra` syntax when importing types from the `kybra` module, and to not rename types with `as`:

Correct:

```python
from kybra import Record

class MyRecord(Record):
    prop1: str
    prop2: int
```

Incorrect:

```python
import kybra

class MyRecord(kybra.Record):
    prop1: str
    prop2: int
```

Incorrect:

```python
from kybra import Record as RenamedRecord

class MyRecord(RenamedRecord):
    prop1: str
    prop2: int
```

We wish to improve this situation in the future to handle the many various ways of importing.

### Treatment as keywords

You should treat Kybra types essentially as keywords, not creating types of the same name elsewhere in your codebase or in other libraries. Any types exported from [this file](https://github.com/demergent-labs/kybra/blob/main/kybra/__init__.py) should be treated thusly.
