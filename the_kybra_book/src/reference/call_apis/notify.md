# notify

This section is a work in progress.

Examples:

-   [cross_canister_calls](https://github.com/demergent-labs/kybra/tree/main/examples/cross_canister_calls)
-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import NotifyResult, Principal, update
from src.canister2.types import Canister2

canister2 = Canister2(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))


@update
def send_notification() -> NotifyResult:
    return canister2.receive_notification("This is the notification").notify()
```
