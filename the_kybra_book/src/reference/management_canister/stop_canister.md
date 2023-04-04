# stop_canister

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import (
    Async,
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
def execute_stop_canister(canister_id: Principal) -> Async[DefaultResult]:
    call_result: CallResult[void] = yield management_canister.stop_canister(
        {"canister_id": canister_id}
    )

    if call_result.err is not None:
        return {"err": call_result.err}

    return {"ok": True}
```
