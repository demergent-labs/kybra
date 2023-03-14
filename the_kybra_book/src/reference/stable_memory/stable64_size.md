# stable64 size

This section is a work in progress.

Examples:

-   [stable_memory](https://github.com/demergent-labs/kybra/tree/main/examples/stable_memory)

```python
from kybra import ic, nat64, query


@query
def stable64_size() -> nat64:
    return ic.stable64_size()
```
