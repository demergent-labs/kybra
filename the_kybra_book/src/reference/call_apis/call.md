# call

This section is a work in progress.

Examples:

-   [bitcoin](https://github.com/demergent-labs/kybra/tree/main/examples/bitcoin)
-   [composite_queries](https://github.com/demergent-labs/kybra/tree/main/examples/composite_queries)
-   [cross_canister_calls](https://github.com/demergent-labs/kybra/tree/main/examples/cross_canister_calls)
-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)
-   [ethereum_json_rpc](https://github.com/demergent-labs/kybra/tree/main/examples/ethereum_json_rpc)
-   [func_types](https://github.com/demergent-labs/kybra/tree/main/examples/func_types)
-   [generators](https://github.com/demergent-labs/kybra/tree/main/examples/generators)
-   [ledger_canister](https://github.com/demergent-labs/kybra/tree/main/examples/ledger_canister)
-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)
-   [rejections](https://github.com/demergent-labs/kybra/tree/main/examples/rejections)
-   [timers](https://github.com/demergent-labs/kybra/tree/main/examples/timers)
-   [whoami](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/whoami)

```python
from kybra import (
    Async,
    Canister,
    CanisterResult,
    nat64,
    Principal,
    update,
    Variant,
)


class TokenCanister(Canister):
    @update
    def transfer(self, to: Principal, amount: nat64) -> nat64:
        ...


token_canister = TokenCanister(Principal.from_str("r7inp-6aaaa-aaaaa-aaabq-cai"))


class PayoutResult(Variant, total=False):
    ok: nat64
    err: str


@update
def payout(to: Principal, amount: nat64) -> Async[PayoutResult]:
    result: CanisterResult[nat64] = yield token_canister.transfer(to, amount)

    if result.err is not None:
        return {"err": result.err}

    return {"ok": result.ok}
```
