# call raw 128

This section is a work in progress.

Examples:

-   [call_raw](https://github.com/demergent-labs/kybra/tree/main/examples/call_raw)

```python
from kybra import (
    Async,
    blob,
    CallResult,
    ic,
    match,
    nat,
    Principal,
    update,
    Variant,
)

class ExecuteCallRaw128Result(Variant, total=False):
    Ok: str
    Err: str


@update
def execute_call_raw128(
    canister_id: Principal, method: str, candid_args: str, payment: nat
) -> Async[ExecuteCallRaw128Result]:
    call_result: CallResult[blob] = yield ic.call_raw128(
        canister_id, method, ic.candid_encode(candid_args), payment
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ic.candid_decode(ok)}, "Err": lambda err: {"Err": err}}
    )
```
