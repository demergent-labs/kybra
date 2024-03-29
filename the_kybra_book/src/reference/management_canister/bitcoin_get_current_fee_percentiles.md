# bitcoin_get_current_fee_percentiles

This section is a work in progress.

Examples:

-   [bitcoin](https://github.com/demergent-labs/kybra/tree/main/examples/bitcoin)

```python
from kybra import Async, CallResult, match, update, Variant, Vec
from kybra.canisters.management import management_canister, MillisatoshiPerByte

BITCOIN_API_CYCLE_COST = 100_000_000


class GetCurrentFeePercentilesResult(Variant, total=False):
    Ok: Vec[MillisatoshiPerByte]
    Err: str


@update
def get_current_fee_percentiles() -> Async[GetCurrentFeePercentilesResult]:
    call_result: CallResult[
        Vec[MillisatoshiPerByte]
    ] = yield management_canister.bitcoin_get_current_fee_percentiles(
        {"network": {"Regtest": None}}
    ).with_cycles(
        BITCOIN_API_CYCLE_COST
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )
```
