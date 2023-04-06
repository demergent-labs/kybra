# notify raw

This section is a work in progress.

Examples:

-   [notify_raw](https://github.com/demergent-labs/kybra/tree/main/examples/notify_raw)

```python
from kybra import ic, Principal, RejectionCode, update, Variant


class SendNotificationResult(Variant, total=False):
    Ok: bool
    Err: RejectionCode


@update
def send_notification() -> SendNotificationResult:
    result = ic.notify_raw(
        Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"),
        "receive_notification",
        ic.candid_encode("()"),
        0,
    )

    return match(
        result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )
```
