# call raw

This section is a work in progress.

Examples:

-   [call_raw](https://github.com/demergent-labs/kybra/tree/main/examples/call_raw)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)

```python
from kybra import (
    Async,
    blob,
    CallResult,
    ic,
    match,
    nat64,
    Principal,
    update,
    Variant,
)


class ExecuteCallRawResult(Variant, total=False):
    Ok: str
    Err: str


@update
def execute_call_raw(
    canister_id: Principal, method: str, candid_args: str, payment: nat64
) -> Async[ExecuteCallRawResult]:
    call_result: CallResult[blob] = yield ic.call_raw(
        canister_id, method, ic.candid_encode(candid_args), payment
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ic.candid_decode(ok)}, "Err": lambda err: {"Err": err}}
    )
```
