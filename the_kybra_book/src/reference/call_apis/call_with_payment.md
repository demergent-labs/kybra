# call with payment

This section is a work in progress.

Examples:

-   [bitcoin](https://github.com/demergent-labs/kybra/tree/main/examples/bitcoin)
-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)
-   [ethereum_json_rpc](https://github.com/demergent-labs/kybra/tree/main/examples/ethereum_json_rpc)
-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)
-   [threshold_ecdsa](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/threshold_ecdsa)

```python
from kybra import Async, blob, CanisterResult, Principal, update, void
from kybra.canisters.management import management_canister
from src.types import DefaultResult


@update
def execute_install_code(
    canister_id: Principal, wasm_module: blob
) -> Async[DefaultResult]:
    canister_result: CanisterResult[void] = yield management_canister.install_code(
        {
            "mode": {"install": None},
            "canister_id": canister_id,
            "wasm_module": wasm_module,
            "arg": bytes(),
        }
    ).with_cycles(100_000_000_000)

    if canister_result.err is not None:
        return {"err": canister_result.err}

    return {"ok": True}
```
