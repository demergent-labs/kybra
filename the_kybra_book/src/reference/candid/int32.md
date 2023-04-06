# int32

This section is a work in progress.

The Kybra type `int32` corresponds to the [Candid type int32](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int32, query


@query
def get_int32() -> int32:
    return 2_147_483_647


@query
def print_int32(int32: int32) -> int32:
    ic.print(type(int32))
    return int32
```

Candid:

```
service: {
    "get_int32": () -> (int32) query;
    "print_int32": (int32) -> (int32) query;
}
```
