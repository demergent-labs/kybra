# provisional_create_canister_with_cycles

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import Async, CallResult, update, Variant
from kybra.canisters.management import (
    CreateCanisterResult,
    management_canister,
    ProvisionalCreateCanisterWithCyclesResult,
)


class ExecuteProvisionalCreateCanisterWithCyclesResult(Variant, total=False):
    ok: CreateCanisterResult
    err: str


@update
def provisional_create_canister_with_cycles() -> (
    Async[ExecuteProvisionalCreateCanisterWithCyclesResult]
):
    canister_result: CallResult[
        ProvisionalCreateCanisterWithCyclesResult
    ] = yield management_canister.provisional_create_canister_with_cycles(
        {"amount": None, "settings": None}
    )

    if canister_result.err is not None:
        return {"err": canister_result.err}

    provisional_create_canister_with_cycles_result = canister_result.ok

    return {"ok": provisional_create_canister_with_cycles_result}
```
