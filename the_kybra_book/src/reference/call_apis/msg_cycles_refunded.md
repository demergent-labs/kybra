# msg cycles refunded

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import (
    Async,
    CallResult,
    ic,
    match,
    nat64,
    Principal,
    Service,
    service_update,
    update,
    Variant,
)


class SendCyclesResult(Variant, total=False):
    Ok: nat64
    Err: str


class Cycles(Service):
    @service_update
    def receive_cycles(self) -> nat64:
        ...


cycles = Cycles(Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai"))


# Reports the number of cycles returned from the Cycles canister
@update
def send_cycles() -> Async[SendCyclesResult]:
    result: CallResult[nat64] = yield cycles.receive_cycles().with_cycles(1_000_000)
    return match(
        result,
        {
            "Ok": lambda _: {"Ok": ic.msg_cycles_refunded()},
            "Err": lambda err: {"Err": err},
        },
    )
```
