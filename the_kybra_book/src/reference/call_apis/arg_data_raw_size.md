# arg data raw size

This section is a work in progress.

Examples:

-   [ic_api](https://github.com/demergent-labs/kybra/blob/main/examples/ic_api)

```python
from kybra import blob, ic, int8, nat32, query


# returns the length of the argument data in bytes
@query
def arg_data_raw_size(arg1: blob, arg2: int8, arg3: bool, arg4: str) -> nat32:
    return ic.arg_data_raw_size()
```
