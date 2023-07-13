# Guard Functions

Guard functions allow you to protect your `query`, `update`, `heartbeat`, `pre_upgrade`, and `inspect_message` methods.

Guard functions return a `GuardResult` that looks like this:

```python
class GuardResult(Variant, total=False):
    Ok: null
    Err: str
```

Returning `Ok` will allow the protected method to proceed with execution. Returning `Err` will reject the canister method call.

Here's a contrived example where the world is protected only 50% of the time:

```python
import random

from kybra import GuardResult, update


def protect_the_world() -> GuardResult:
    if random.random() > 0.5:
        return {"Ok": None}
    else:
        return {"Err": "The world must be protected"}


@update(guard=protect_the_world)
def hello_world() -> str:
    return "Hello world!"
```
