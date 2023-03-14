# deposit_cycles

This section is a work in progress.

Examples:

-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)

```python
@update
def execute_deposit_cycles(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CanisterResult[void] = yield management_canister.deposit_cycles(
        {"canister_id": canister_id}
    ).with_cycles(1_000_000)

    if canister_result.err is not None:
        return {"err": canister_result.err}

    return {"ok": True}
```
