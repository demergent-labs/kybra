# vec

This section is a work in progress.

The Kybra type `Vec` corresponds to the [Candid type vec](https://internetcomputer.org/docs/current/references/candid-ref#type-vec-t) and will become an array of the specified type at runtime.

Python:

```python
from kybra import int32, query, Vec


@query
def get_numbers() -> Vec[int32]:
    return [0, 1, 2, 3]
```

Candid:

```
service: {
    "get_numbers": () -> (vec int32) query;
}
```
