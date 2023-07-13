# ecdsa_public_key

This section is a work in progress.

Examples:

-   [threshold_ecdsa](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/threshold_ecdsa)

```python
from kybra import Async, blob, CallResult, ic, match, Record, update, Variant
from kybra.canisters.management import management_canister, EcdsaPublicKeyResult


class PublicKey(Record):
    public_key: blob


class PublicKeyResult(Variant, total=False):
    Ok: PublicKey
    Err: str


@update
def public_key() -> Async[PublicKeyResult]:
    caller = ic.caller().bytes
    call_result: CallResult[
        EcdsaPublicKeyResult
    ] = yield management_canister.ecdsa_public_key(
        {
            "canister_id": None,
            "derivation_path": [caller],
            "key_id": {"curve": {"secp256k1": None}, "name": "dfx_test_key"},
        }
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )
```
