# provisional_top_up_canister

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import (
    Async,
    CallResult,
    nat,
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
def provisional_top_up_canister(
    canister_id: Principal, amount: nat
) -> Async[DefaultResult]:
    canister_result: CallResult[
        void
    ] = yield management_canister.provisional_top_up_canister(
        {"canister_id": canister_id, "amount": amount}
    )

    if canister_result.err is not None:
        return {"err": canister_result.err}

    return {"ok": True}
```
