# candid encode

This section is a work in progress.

Examples:

-   [call_raw](https://github.com/demergent-labs/kybra/tree/main/examples/call_raw)
-   [candid_encoding](https://github.com/demergent-labs/kybra/tree/main/examples/candid_encoding)
-   [manual_reply](https://github.com/demergent-labs/kybra/tree/main/examples/manual_reply)
-   [notify_raw](https://github.com/demergent-labs/kybra/tree/main/examples/notify_raw)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)

```python
from kybra import blob, ic, query


# encodes a Candid string to Candid bytes
@query
def candid_encode(candid_string: str) -> blob:
    return ic.candid_encode(candid_string)
```
