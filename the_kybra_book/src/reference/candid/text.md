# text

This section is a work in progress.

The Python type `str` and the Kybra type `text` both correspond to the [Candid type text](https://internetcomputer.org/docs/current/references/candid-ref#type-text) and will become a [Python str](https://docs.python.org/3/library/stdtypes.html#textseq) at runtime.

Python:

```python
from kybra import ic, query


@query
def get_string() -> str:
    return "Hello world!"


@query
def print_string(string: str) -> str:
    ic.print(type(string))
    return string

```

Candid:

```
service: {
    "get_string": () -> (text) query;
    "print_string": (text) -> (text) query;
}
```
