# reject

This section is a work in progress.

Examples:

-   [ic_api](https://github.com/demergent-labs/kybra/tree/main/examples/ic_api)
-   [manual_reply](https://github.com/demergent-labs/kybra/tree/main/examples/manual_reply)
-   [rejections](https://github.com/demergent-labs/kybra/tree/main/examples/rejections)

```python
from kybra import empty, ic, manual, query


@query
def reject(message: str) -> manual[empty]:
    ic.reject(message)
```
