# canister_status

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import Async, CallResult, update, Variant
from kybra.canisters.management import (
    CanisterStatusArgs,
    CanisterStatusResult,
    management_canister,
)


class GetCanisterStatusResult(Variant, total=False):
    Ok: CanisterStatusResult
    Err: str


@update
def get_canister_status(args: CanisterStatusArgs) -> Async[GetCanisterStatusResult]:
    canister_status_result_call_result: CallResult[
        CanisterStatusResult
    ] = yield management_canister.canister_status({"canister_id": args["canister_id"]})

    return match(
        canister_status_result_call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )
```
