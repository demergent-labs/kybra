# notify

This section is a work in progress.

Examples:

-   [cross_canister_calls](https://github.com/demergent-labs/kybra/tree/main/examples/cross_canister_calls)
-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import NotifyResult, Principal, Service, service_update, update, void


class Canister2(Service):
    @service_update
    def receive_notification(self, message: str) -> void:
        ...


canister2 = Canister2(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))


@update
def send_notification() -> NotifyResult:
    return canister2.receive_notification("This is the notification").notify()
```
