# canister balance 128

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)
-   [ic_api](https://github.com/demergent-labs/kybra/tree/main/examples/ic_api)

```python
from kybra import ic, nat, query


# returns the amount of cycles available in the canister
@query
def canister_balance128() -> nat:
    return ic.canister_balance128()
```
