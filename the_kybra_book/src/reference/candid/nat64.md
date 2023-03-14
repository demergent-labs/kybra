# nat64

This section is a work in progress.

The Kybra type `nat64` corresponds to the [Candid type nat64](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat64, query

@query
def get_nat64() -> nat64:
    return 18_446_744_073_709_551_615

@query
def print_nat64(nat64: nat64) -> nat64:
    ic.print(type(nat64))
    return nat64
```

Candid:

```python
service: {
    "get_nat64": () -> (nat64) query;
    "print_nat64": (nat64) -> (nat64) query;
}
```
