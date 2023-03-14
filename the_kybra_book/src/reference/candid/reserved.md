# reserved

This section is a work in progress.

The Kybra type `reserved` corresponds to the [Candid type reserved](https://internetcomputer.org/docs/current/references/candid-ref#type-reserved) and will become the [Python Null Object](https://docs.python.org/3/library/stdtypes.html#the-null-object) at runtime.

Python:

```python
from kybra import ic, query, reserved

@query
def get_reserved() -> reserved:
    return 'anything'

@query
def print_reserved(reserved: reserved) -> reserved:
    ic.print(type(reserved))
    return reserved
```

Candid:

```python
service: {
    "get_reserved": () -> (reserved) query;
    "print_reserved": (reserved) -> (reserved) query;
}
```
