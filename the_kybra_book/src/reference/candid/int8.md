# int8

This section is a work in progress.

The Kybra type `int8` corresponds to the [Candid type int8](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int8, query


@query
def get_int8() -> int8:
    return 127


@query
def print_int8(int8: int8) -> int8:
    ic.print(type(int8))
    return int8
```

Candid:

```
service: {
    "get_int8": () -> (int8) query;
    "print_int8": (int8) -> (int8) query;
}
```
