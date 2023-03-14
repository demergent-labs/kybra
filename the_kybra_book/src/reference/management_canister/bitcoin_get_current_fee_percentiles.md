# bitcoin_get_current_fee_percentiles

This section is a work in progress.

Examples:

-   [bitcoin](https://github.com/demergent-labs/kybra/tree/main/examples/bitcoin)

```python
from kybra import Async, CanisterResult, update, Variant
from kybra.canisters.management.bitcoin import MillisatoshiPerByte
from kybra.canisters.management import management_canister

BITCOIN_API_CYCLE_COST = 100_000_000


class GetCurrentFeePercentilesResult(Variant, total=False):
    ok: list[MillisatoshiPerByte]
    err: str


@update
def get_current_fee_percentiles() -> Async[GetCurrentFeePercentilesResult]:
    canister_result: CanisterResult[
        list[MillisatoshiPerByte]
    ] = yield management_canister.bitcoin_get_current_fee_percentiles(
        {"network": {"Regtest": None}}
    ).with_cycles(
        BITCOIN_API_CYCLE_COST
    )

    if canister_result.err is not None:
        return {"err": canister_result.err}

    return {"ok": canister_result.ok}
```
