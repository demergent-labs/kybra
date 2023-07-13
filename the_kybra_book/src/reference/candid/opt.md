# opt

This section is a work in progress.

The Kybra type `Opt` corresponds to the [Candid type opt](https://internetcomputer.org/docs/current/references/candid-ref#type-opt-t) and will become the enclosed Python type or None at runtime.

Python:

```python
from kybra import Opt, query


@query
def get_opt_some() -> Opt[bool]:
    return True


@query
def get_opt_none() -> Opt[bool]:
    return None
```

Candid:

```
service: {
    "get_opt_some": () -> (opt bool) query;
    "get_opt_none": () -> (opt bool) query;
}
```
