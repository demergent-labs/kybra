# int64

This section is a work in progress.

The Kybra type `int64` corresponds to the [Candid type int64](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int64, query


@query
def get_int64() -> int64:
    return 9_223_372_036_854_775_807


@query
def print_int64(int64: int64) -> int64:
    ic.print(type(int64))
    return int64
```

Candid:

```
service: {
    "get_int64": () -> (int64) query;
    "print_int64": (int64) -> (int64) query;
}
```
