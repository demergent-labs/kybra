# create_canister

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import Async, CallResult, match, update, Variant
from kybra.canisters.management import CreateCanisterResult, management_canister


class ExecuteCreateCanisterResult(Variant, total=False):
    Ok: CreateCanisterResult
    Err: str


@update
def execute_create_canister() -> Async[ExecuteCreateCanisterResult]:
    create_canister_result_call_result: CallResult[
        CreateCanisterResult
    ] = yield management_canister.create_canister({"settings": None}).with_cycles(
        50_000_000_000_000
    )

    return match(
        create_canister_result_call_result,
        {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}},
    )
```
