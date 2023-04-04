# create_canister

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import Async, CallResult, update, Variant
from kybra.canisters.management import CreateCanisterResult, management_canister


class ExecuteCreateCanisterResult(Variant, total=False):
    ok: CreateCanisterResult
    err: str


@update
def execute_create_canister() -> Async[ExecuteCreateCanisterResult]:
    create_canister_result_canister_result: CallResult[
        CreateCanisterResult
    ] = yield management_canister.create_canister({"settings": None}).with_cycles(
        50_000_000_000_000
    )

    if create_canister_result_canister_result.err is not None:
        return {"err": create_canister_result_canister_result.err}

    create_canister_result = create_canister_result_canister_result.ok

    return {"ok": create_canister_result}
```
