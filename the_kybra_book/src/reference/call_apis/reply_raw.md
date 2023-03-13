# reply raw

This section is a work in progress.

Examples:

-   [manual_reply](https://github.com/demergent-labs/kybra/tree/main/examples/manual_reply)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)

```python
from kybra import blob, ic, manual, null, query, Record, Variant


class RawReply(Record):
    int: int
    text: str
    bool: bool
    blob: blob
    variant: "Options"


class Options(Variant, total=False):
    Small: null
    Medium: null
    Large: null


@query
def reply_raw() -> manual[RawReply]:
    ic.reply_raw(
        ic.candid_encode(
            '(record { "int" = 42; "text" = "text"; "bool" = true; "blob" = blob "Surprise!"; "variant" = variant { Medium } })'
        )
    )
```
