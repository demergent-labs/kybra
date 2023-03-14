# stable bytes

This section is a work in progress.

Examples:

-   [stable_memory](https://github.com/demergent-labs/kybra/tree/main/examples/stable_memory)

```python
from kybra import blob, ic, query


@query
def stable_bytes() -> blob:
    return ic.stable_bytes()
```
