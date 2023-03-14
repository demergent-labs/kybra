# empty

This section is a work in progress.

The Kybra type `empty` corresponds to the [Candid type empty](https://internetcomputer.org/docs/current/references/candid-ref#type-empty) and has no Python value at runtime.

Python:

```python
from kybra import empty, ic, query

def get_empty() -> empty:
    raise Exception('Anything you want')

# Note: It is impossible to call this function because it requires an argument
# but there is no way to pass an "empty" value as an argument.
@query
def print_empty(empty: empty) -> empty:
    ic.print(type(empty))
    raise Exception('Anything you want')
```

Candid:

```python
service: {
    "get_empty": () -> (empty) query;
    "print_empty": (empty) -> (empty) query;
}
```
