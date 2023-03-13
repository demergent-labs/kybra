# stable size

This section is a work in progress.

Examples:

-   [stable_memory](https://github.com/demergent-labs/kybra/tree/main/examples/stable_memory)

```python
from kybra import ic, nat32, query


@query
def stable_size() -> nat32:
    return ic.stable_size()
```
