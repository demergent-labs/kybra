# float64

This section is a work in progress.

The Kybra type `float64` corresponds to the [Candid type float64](https://internetcomputer.org/docs/current/references/candid-ref#type-float32-and-float64) and will become a [Python float](https://docs.python.org/3/library/functions.html#float) at runtime.

Python:

```python
import math

from kybra import float64, ic, query


@query
def get_float64() -> float64:
    return math.e

@query
def print_float64(float64: float64) -> float64:
    ic.print(type(float64))
    return float64
```

Candid:

```python
service: {
    "get_float64": () -> (float64) query;
    "print_float64": (float64) -> (float64) query;
}
```
