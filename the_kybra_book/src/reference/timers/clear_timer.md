# clear timer

This section is a work in progress.

Examples:

-   [timers](https://github.com/demergent-labs/kybra/tree/main/examples/timers)

```python
from kybra import ic, TimerId, update


@update
def clear_timer(timer_id: TimerId):
    ic.clear_timer(timer_id)
    ic.print(f"timer {timer_id} cancelled")
```
