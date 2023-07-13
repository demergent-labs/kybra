# sign_with_ecdsa

This section is a work in progress.

Examples:

-   [threshold_ecdsa](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/threshold_ecdsa)

```python
from kybra import Async, blob, CallResult, ic, match, Record, update, Variant
from kybra.canisters.management import (
    management_canister,
    SignWithEcdsaResult,
)


class Signature(Record):
    signature: blob


class SignResult(Variant, total=False):
    Ok: Signature
    Err: str


@update
def sign(message_hash: blob) -> Async[SignResult]:
    if len(message_hash) != 32:
        ic.trap("message_hash must be 32 bytes")

    caller = ic.caller().bytes

    call_result: CallResult[
        SignWithEcdsaResult
    ] = yield management_canister.sign_with_ecdsa(
        {
            "message_hash": message_hash,
            "derivation_path": [caller],
            "key_id": {"curve": {"secp256k1": None}, "name": "dfx_test_key"},
        }
    ).with_cycles(
        10_000_000_000
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )
```
