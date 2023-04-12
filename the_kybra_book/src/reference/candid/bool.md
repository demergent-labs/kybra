# bool

This section is a work in progress.

The Python type `bool` corresponds to the [Candid type bool](https://internetcomputer.org/docs/current/references/candid-ref#type-bool) and will become a [Python Boolean Value](https://docs.python.org/3/library/stdtypes.html#boolean-values) at runtime.

Python:

```python
from kybra import ic, query


@query
def get_bool() -> bool:
    return True


@query
def print_bool(bool: bool) -> bool:
    ic.print(type(bool))
    return bool
```

Candid:

```
service: {
    "get_bool": () -> (bool) query;
    "print_bool": (bool) -> (bool) query;
}
```
