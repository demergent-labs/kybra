# set timer interval

This section is a work in progress.

Examples:

-   [timers](https://github.com/demergent-labs/kybra/tree/main/examples/timers)

```python
from kybra import (
    Duration,
    ic,
    TimerId,
    update,
)

counter = 0


@update
def set_timer_interval(interval: Duration) -> TimerId:
    return ic.set_timer_interval(interval, timer_callback)


def timer_callback():
    global counter
    counter += 1

    ic.print(f"timer_callback: {counter}")
```
