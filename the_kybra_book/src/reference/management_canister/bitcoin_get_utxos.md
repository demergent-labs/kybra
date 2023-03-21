# bitcoin_get_utxos

This section is a work in progress.

Examples:

-   [bitcoin](https://github.com/demergent-labs/kybra/tree/main/examples/bitcoin)

```python
from kybra import Async, CanisterResult, update, Variant
from kybra.canisters.management import GetUtxosResult, management_canister

BITCOIN_API_CYCLE_COST = 100_000_000


class ExecuteGetUtxosResult(Variant, total=False):
    ok: GetUtxosResult
    err: str


@update
def get_utxos(address: str) -> Async[ExecuteGetUtxosResult]:
    canister_result: CanisterResult[
        GetUtxosResult
    ] = yield management_canister.bitcoin_get_utxos(
        {"address": address, "filter": None, "network": {"Regtest": None}}
    ).with_cycles(
        BITCOIN_API_CYCLE_COST
    )

    if canister_result.err is not None:
        return {"err": canister_result.err}

    return {"ok": canister_result.ok}
```
