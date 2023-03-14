# null

This section is a work in progress.

The Python type `None` and the Kybra type `null` both correspond to the [Candid type null](https://internetcomputer.org/docs/current/references/candid-ref#type-null) and will become the [Python Null Object](https://docs.python.org/3/library/stdtypes.html#the-null-object) at runtime.

Python:

```python
from kybra import ic, query

@query
def get_null() -> None:
    return None

@query
def print_null(none: None) -> None:
    ic.print(type(none))
    return none
```

Candid:

```python
service: {
    "get_null": () -> (null) query;
    "print_null": (null) -> (null) query;
}
```
