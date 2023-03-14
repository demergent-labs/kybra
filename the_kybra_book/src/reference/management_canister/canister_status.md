# canister_status

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import Async, CanisterResult, update, Variant
from kybra.canisters.management import (
    CanisterStatusArgs,
    CanisterStatusResult,
    management_canister,
)


class GetCanisterStatusResult(Variant, total=False):
    ok: CanisterStatusResult
    err: str


@update
def get_canister_status(args: CanisterStatusArgs) -> Async[GetCanisterStatusResult]:
    canister_status_result_canister_result: CanisterResult[
        CanisterStatusResult
    ] = yield management_canister.canister_status({"canister_id": args["canister_id"]})

    if canister_status_result_canister_result.err is not None:
        return {"err": canister_status_result_canister_result.err}

    canister_status_result = canister_status_result_canister_result.ok

    return {"ok": canister_status_result}
```
