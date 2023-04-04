# bitcoin_send_transaction

This section is a work in progress.

Examples:

-   [bitcoin](https://github.com/demergent-labs/kybra/tree/main/examples/bitcoin)

```python
from kybra import Async, blob, CallResult, null, update, Variant, void
from kybra.canisters.management import management_canister

BITCOIN_BASE_TRANSACTION_COST = 5_000_000_000
BITCOIN_CYCLE_COST_PER_TRANSACTION_BYTE = 20_000_000


class SendTransactionResult(Variant, total=False):
    ok: null
    err: str


@update
def send_transaction(transaction: blob) -> Async[SendTransactionResult]:
    transaction_fee = (
        BITCOIN_BASE_TRANSACTION_COST
        + len(transaction) * BITCOIN_CYCLE_COST_PER_TRANSACTION_BYTE
    )

    canister_result: CallResult[
        void
    ] = yield management_canister.bitcoin_send_transaction(
        {"transaction": transaction, "network": {"Regtest": None}}
    ).with_cycles(
        transaction_fee
    )

    if canister_result.err is not None:
        return {"err": canister_result.err}

    return {"ok": canister_result.ok}
```
