# blob

This section is a work in progress.

The Kybra type `blob` corresponds to the [Candid type blob](https://internetcomputer.org/docs/current/references/candid-ref#type-blob) and will become a [Python bytes](https://docs.python.org/3/library/stdtypes.html#bytes) at runtime.

Python:

```Python
from kybra import blob, ic, query

@query
def get_blob() -> blob:
    return bytes([68, 73, 68, 76, 0, 0])

@query
def print_blob(blob: blob) -> blob:
    ic.print(type(blob))
    return blob
```

Candid:

```python
service: {
    "get_blob": () -> (blob) query;
    "print_blob": (blob) -> (blob) query;
}
```
