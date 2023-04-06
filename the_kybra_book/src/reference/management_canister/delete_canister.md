# delete_canister

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
    Ok: bool
    Err: str


@update
def execute_delete_canister(canister_id: Principal) -> Async[DefaultResult]:
    call_result: CallResult[void] = yield management_canister.delete_canister(
        {"canister_id": canister_id}
    )

    return match(
        call_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )
```
