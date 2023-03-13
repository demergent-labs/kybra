# msg cycles accept 128

This section is a work in progress.

Examples:

-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)

```python
from kybra import ic, nat, update


@update
def receive_cycles128() -> nat:
    return ic.msg_cycles_accept128(ic.msg_cycles_available128() // 2)
```
