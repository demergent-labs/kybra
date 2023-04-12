# msg cycles refunded 128

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import (
    Async,
    CallResult,
    ic,
    match,
    nat,
    Principal,
    Service,
    service_update,
    update,
    Variant,
)


class SendCyclesResult128(Variant, total=False):
    Ok: nat
    Err: str


class Cycles(Service):
    @service_update
    def receive_cycles128(self) -> nat:
        ...


cycles = Cycles(Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai"))


# Reports the number of cycles returned from the Cycles canister
@update
def send_cycles128() -> Async[SendCyclesResult128]:
    result: CallResult[nat] = yield cycles.receive_cycles128().with_cycles128(1_000_000)

    return match(
        result,
        {
            "Ok": lambda _: {"Ok": ic.msg_cycles_refunded128()},
            "Err": lambda err: {"Err": err},
        },
    )
```
