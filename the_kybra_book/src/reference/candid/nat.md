# nat

This section is a work in progress.

The Kybra type `nat` corresponds to the [Candid type nat](https://internetcomputer.org/docs/current/references/candid-ref#type-nat) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat, query


@query
def get_nat() -> nat:
    return 340_282_366_920_938_463_463_374_607_431_768_211_455


@query
def print_nat(nat: nat) -> nat:
    ic.print(type(nat))
    return nat
```

Candid:

```
service: {
    "get_nat": () -> (nat) query;
    "print_nat": (nat) -> (nat) query;
}
```
