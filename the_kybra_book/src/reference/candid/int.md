# int

This section is a work in progress.

The Kybra type `int` corresponds to the [Candid type int](https://internetcomputer.org/docs/current/references/candid-ref#type-int) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, query


@query
def get_int() -> int:
    return 170_141_183_460_469_231_731_687_303_715_884_105_727


@query
def print_int(int: int) -> int:
    ic.print(type(int))
    return int
```

Candid:

```
service: {
    "get_int": () -> (int) query;
    "print_int": (int) -> (int) query;
}
```
