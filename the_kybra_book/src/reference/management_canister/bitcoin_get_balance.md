# bitcoin_get_balance

This section is a work in progress.

Examples:

-   [bitcoin](https://github.com/demergent-labs/kybra/tree/main/examples/bitcoin)

```python
from kybra import Async, CallResult, update, Variant
from kybra.canisters.management import management_canister, Satoshi

BITCOIN_API_CYCLE_COST = 100_000_000


class ExecuteGetBalanceResult(Variant, total=False):
    ok: Satoshi
    err: str


@update
def get_balance(address: str) -> Async[ExecuteGetBalanceResult]:
    call_result: CallResult[
        Satoshi
    ] = yield management_canister.bitcoin_get_balance(
        {"address": address, "min_confirmations": None, "network": {"Regtest": None}}
    ).with_cycles(
        BITCOIN_API_CYCLE_COST
    )

    if call_result.err is not None:
        return {"err": call_result.err}

    return {"ok": call_result.ok}
```
