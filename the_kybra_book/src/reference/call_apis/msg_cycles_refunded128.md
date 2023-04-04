# msg cycles refunded 128

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import (
    Async,
    CallResult,
    ic,
    nat,
    update,
    Variant,
)
from src.cycles.types import cycles


class SendCyclesResult128(Variant, total=False):
    ok: nat
    err: str


# Reports the number of cycles returned from the Cycles canister
@update
def send_cycles128() -> Async[SendCyclesResult128]:
    result: CallResult[nat] = yield cycles.receive_cycles128().with_cycles128(
        1_000_000
    )

    if result.err is not None:
        return {"err": result.err}

    return {"ok": ic.msg_cycles_refunded128()}
```
