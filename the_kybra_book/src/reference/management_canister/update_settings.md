# update_settings

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import Async, CanisterResult, Principal, update, Variant, void
from kybra.canisters.management import management_canister


class DefaultResult(Variant, total=False):
    ok: bool
    err: str


@update
def execute_update_settings(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CanisterResult[void] = yield management_canister.update_settings(
        {
            "canister_id": canister_id,
            "settings": {
                "controllers": None,
                "compute_allocation": 1,
                "memory_allocation": 3_000_000,
                "freezing_threshold": 2_000_000,
            },
        }
    )

    if canister_result.err is not None:
        return {"err": canister_result.err}

    return {"ok": True}
```
