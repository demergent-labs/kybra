# int16

This section is a work in progress.

The Kybra type `int16` corresponds to the [Candid type int16](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int16, query

@query
def get_int16() -> int16:
    return 32_767

@query
def print_int16(int16: int16) -> int16:
    ic.print(type(int16))
    return int16
```

Candid:

```python
service: {
    "get_int16": () -> (int16) query;
    "print_int16": (int16) -> (int16) query;
}
```
