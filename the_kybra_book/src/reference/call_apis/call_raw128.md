# call raw 128

This section is a work in progress.

Examples:

-   [call_raw](https://github.com/demergent-labs/kybra/tree/main/examples/call_raw)

```python
from kybra import (
    Async,
    blob,
    CanisterResult,
    ic,
    nat,
    Principal,
    update,
    Variant,
)

class ExecuteCallRaw128Result(Variant, total=False):
    ok: str
    err: str


@update
def execute_call_raw128(
    canister_id: Principal, method: str, candid_args: str, payment: nat
) -> Async[ExecuteCallRaw128Result]:
    canister_result: CanisterResult[blob] = yield ic.call_raw128(
        canister_id, method, ic.candid_encode(candid_args), payment
    )

    if canister_result.err is not None:
        return {"err": canister_result.err}

    return {"ok": ic.candid_decode(canister_result.ok)}

```
