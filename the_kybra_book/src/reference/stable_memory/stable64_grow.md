# stable64 grow

This section is a work in progress.

Examples:

-   [stable_memory](https://github.com/demergent-labs/kybra/tree/main/examples/stable_memory)

```python
from kybra import ic, nat64, Stable64GrowResult, update


@update
def stable_grow(new_pages: nat64) -> Stable64GrowResult:
    return ic.stable_grow(new_pages)
```
