# install_code

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import (
    Async,
    blob,
    CallResult,
    Principal,
    update,
    Variant,
    void,
)
from kybra.canisters.management import management_canister


class DefaultResult(Variant, total=False):
    ok: bool
    err: str


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
    ).with_cycles(100_000_000_000)

    if call_result.err is not None:
        return {"err": call_result.err}

    return {"ok": True}
```
