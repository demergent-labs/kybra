# stable write

This section is a work in progress.

Examples:

-   [stable_memory](https://github.com/demergent-labs/kybra/tree/main/examples/stable_memory)

```python
from kybra import blob, ic, nat32, update, void


@update
def stable_write(offset: nat32, buf: blob) -> void:
    ic.stable_write(offset, buf)
```
