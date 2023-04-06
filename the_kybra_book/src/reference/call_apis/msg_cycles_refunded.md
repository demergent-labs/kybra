# msg cycles refunded

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import (
    Async,
    CallResult,
    ic,
    nat64,
    update,
    Variant,
)
from src.cycles.types import cycles


class SendCyclesResult(Variant, total=False):
    ok: nat64
    err: str


# Reports the number of cycles returned from the Cycles canister
@update
def send_cycles() -> Async[SendCyclesResult]:
    result: CallResult[nat64] = yield cycles.receive_cycles().with_cycles(1_000_000)

    if result.err is not None:
        return {"err": result.err}

    return {"ok": ic.msg_cycles_refunded()}
```
