# stable read

This section is a work in progress.

Examples:

-   [stable_memory](https://github.com/demergent-labs/kybra/tree/main/examples/stable_memory)

```python
from kybra import blob, ic, nat32, query


@query
def stable_read(offset: nat32, length: nat32) -> blob:
    return ic.stable_read(offset, length)
```
