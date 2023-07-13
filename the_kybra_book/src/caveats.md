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

## init and post_upgrade params

If you add parameters to your `init` or `post_upgrade` methods, you will need to manually add these parameters to the service in your Candid file for the first deploy to succeed. Any time you change these parameters, you will need to manually add these changes into your Candid file.

It is also recommended to start clean (`dfx start --clean`) when first deploying after changing these parameters.

Here's an example of a basic Candid file with parameters `text`, `bool`, and `int32`:

```
service : (text, bool, int32) -> {}
```

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
