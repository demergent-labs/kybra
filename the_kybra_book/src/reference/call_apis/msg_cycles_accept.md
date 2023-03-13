# msg cycles accept

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import ic, nat64, update


@update
def receive_cycles() -> nat64:
    return ic.msg_cycles_accept(ic.msg_cycles_available() // 2)
```
