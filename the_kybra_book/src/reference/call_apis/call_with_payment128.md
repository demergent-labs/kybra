# call with payment 128

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import Async, blob, CallResult, match, Principal, update, Variant, void
from kybra.canisters.management import management_canister


class DefaultResult(Variant, total=False):
    Ok: bool
    Err: str


@update
def execute_install_code(
    canister_id: Principal, wasm_module: blob
) -> Async[DefaultResult]:
    call_result: CallResult[void] = yield management_canister.install_code(
        {
            "mode": {"install": None},
            "canister_id": canister_id,
            "wasm_module": wasm_module,
            "arg": bytes(),
        }
    ).with_cycles128(100_000_000_000)

    return match(
        call_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )
```
