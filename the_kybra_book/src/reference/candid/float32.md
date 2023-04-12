# float32

This section is a work in progress.

The Kybra type `float32` corresponds to the [Candid type float32](https://internetcomputer.org/docs/current/references/candid-ref#type-float32-and-float64) and will become a [Python float](https://docs.python.org/3/library/functions.html#float) at runtime.

Python:

```python
import math

from kybra import float32, ic, query


@query
def get_float32() -> float32:
    return math.pi


@query
def print_float32(float32: float32) -> float32:
    ic.print(type(float32))
    return float32
```

Candid:

```
service: {
    "get_float32": () -> (float32) query;
    "print_float32": (float32) -> (float32) query;
}
```
