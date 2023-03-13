# stable64 write

This section is a work in progress.

Examples:

-   [stable_memory](https://github.com/demergent-labs/kybra/tree/main/examples/stable_memory)

```python
from kybra import blob, ic, nat64, update, void


@update
def stable64_write(offset: nat64, buf: blob) -> void:
    ic.stable64_write(offset, buf)
```
