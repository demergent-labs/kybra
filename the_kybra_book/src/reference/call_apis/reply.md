# reply

This section is a work in progress.

Examples:

-   [composite_queries](https://github.com/demergent-labs/kybra/tree/main/examples/composite_queries)
-   [manual_reply](https://github.com/demergent-labs/kybra/tree/main/examples/manual_reply)

```python
from kybra import blob, ic, Manual, update


@update
def update_blob() -> Manual[blob]:
    ic.reply(bytes([83, 117, 114, 112, 114, 105, 115, 101, 33]))
```
