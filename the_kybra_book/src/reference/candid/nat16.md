# nat16

This section is a work in progress.

The Kybra type `nat16` corresponds to the [Candid type nat16](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat16, query


@query
def get_nat16() -> nat16:
    return 65_535


@query
def print_nat16(nat16: nat16) -> nat16:
    ic.print(type(nat16))
    return nat16
```

Candid:

```
service: {
    "get_nat16": () -> (nat16) query;
    "print_nat16": (nat16) -> (nat16) query;
}
```
