# set certified data

This section is a work in progress.

Examples:

-   [ic_api](https://github.com/demergent-labs/azle/tree/main/examples/ic_api)

```python
from kybra import blob, ic, update, void


# sets up to 32 bytes of certified data
@update
def set_certified_data(data: blob) -> void:
    ic.set_certified_data(data)
```
