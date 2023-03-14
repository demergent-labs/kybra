# canister balance

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)
-   [ic_api](https://github.com/demergent-labs/kybra/tree/main/examples/ic_api)

```python
from kybra import ic, nat64, query


# returns the amount of cycles available in the canister
@query
def canister_balance() -> nat64:
    return ic.canister_balance()
```
