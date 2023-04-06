# nat32

This section is a work in progress.

The Kybra type `nat32` corresponds to the [Candid type nat32](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat32, query


@query
def get_nat32() -> nat32:
    return 4_294_967_295


@query
def print_nat32(nat32: nat32) -> nat32:
    ic.print(type(nat32))
    return nat32
```

Candid:

```
service: {
    "get_nat32": () -> (nat32) query;
    "print_nat32": (nat32) -> (nat32) query;
}
```
