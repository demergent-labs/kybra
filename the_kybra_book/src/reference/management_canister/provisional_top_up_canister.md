# provisional_top_up_canister

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
from kybra import (
    Async,
    CallResult,
    match,
    nat,
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
def provisional_top_up_canister(
    canister_id: Principal, amount: nat
) -> Async[DefaultResult]:
    call_result: CallResult[
        void
    ] = yield management_canister.provisional_top_up_canister(
        {"canister_id": canister_id, "amount": amount}
    )

    return match(
        call_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )
```
