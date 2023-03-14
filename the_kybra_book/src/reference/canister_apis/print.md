# print

This section is a work in progress.

Examples:

-   [ic_api](https://github.com/demergent-labs/kybra/tree/main/examples/ic_api)
-   [null_example](https://github.com/demergent-labs/kybra/tree/main/examples/null_example)

```python
from kybra import ic, query


# prints a message through the local replica's output
@query
def print(message: str) -> bool:
    ic.print(message)

    return True
```
