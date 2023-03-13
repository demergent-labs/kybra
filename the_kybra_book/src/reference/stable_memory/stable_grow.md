# stable grow

This section is a work in progress.

Examples:

-   [stable_memory](https://github.com/demergent-labs/kybra/tree/main/examples/stable_memory)

```python
from kybra import ic, nat32, StableGrowResult, update


@update
def stable_grow(new_pages: nat32) -> StableGrowResult:
    return ic.stable_grow(new_pages)
```
