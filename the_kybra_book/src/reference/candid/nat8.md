# nat8

This section is a work in progress.

The Kybra type `nat8` corresponds to the [Candid type nat8](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat8, query


@query
def get_nat8() -> nat8:
    return 255


@query
def print_nat8(nat8: nat8) -> nat8:
    ic.print(type(nat8))
    return nat8
```

Candid:

```
service: {
    "get_nat8": () -> (nat8) query;
    "print_nat8": (nat8) -> (nat8) query;
}
```
