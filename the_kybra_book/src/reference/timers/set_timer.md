# set timer

This section is a work in progress.

Examples:

-   [timers](https://github.com/demergent-labs/azle/tree/main/examples/timers)

```python
from kybra import (
    Duration,
    ic,
    TimerId,
    update,
)


@update
def set_timer(delay: Duration) -> TimerId:
    return ic.set_timer(delay, timer_callback)


def timer_callback():
    ic.print("timer_callback")

```
