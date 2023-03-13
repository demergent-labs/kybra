# arg data raw

This section is a work in progress.

Examples:

-   [ic_api](https://github.com/demergent-labs/kybra/blob/main/examples/ic_api)

```python
from kybra import blob, ic, int8, query


# returns the argument data as bytes.
@query
def arg_data_raw(arg1: blob, arg2: int8, arg3: bool, arg4: str) -> blob:
    return ic.arg_data_raw()
```
