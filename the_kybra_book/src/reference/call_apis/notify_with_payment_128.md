# notify with payment 128

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import NotifyResult, Principal, update
from src.cycles.types import Cycles


cycles = Cycles(Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai"))


@update
def send_cycles128_notify() -> NotifyResult:
    return cycles.receive_cycles128().with_cycles128(1_000_000).notify()
```
