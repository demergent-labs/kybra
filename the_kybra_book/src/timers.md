# Timers

This chapter is a work in progress.

```python
from kybra import (
    Async,
    blob,
    CanisterResult,
    Duration,
    ic,
    nat8,
    query,
    Record,
    TimerId,
    update,
)
from kybra.canisters.management import management_canister


class StatusReport(Record):
    single: bool
    inline: nat8
    capture: str
    repeat: nat8
    single_cross_canister: blob
    repeat_cross_canister: blob


class TimerIds(Record):
    single: TimerId
    inline: TimerId
    capture: TimerId
    repeat: TimerId
    single_cross_canister: TimerId
    repeat_cross_canister: TimerId


status: StatusReport = {
    "single": False,
    "inline": 0,
    "capture": "",
    "repeat": 0,
    "single_cross_canister": bytes(),
    "repeat_cross_canister": bytes(),
}


@update
def clear_timer(timer_id: TimerId):
    ic.clear_timer(timer_id)
    ic.print(f"timer {timer_id} cancelled")


@update
def set_timers(delay: Duration, interval: Duration) -> TimerIds:
    captured_value = "🚩"

    single_id = ic.set_timer(delay, one_time_timer_callback)

    # Note: You cannot set global variables from within a lambda but you can
    # call a function that sets a global variable. So we've moved the "setting"
    # functionality out into helper functions while the printing is kept here in
    # the lambda.

    inline_id = ic.set_timer(
        delay,
        lambda: update_inline_status() or ic.print("Inline timer called"),
    )

    capture_id = ic.set_timer(
        delay,
        lambda: update_capture_status(captured_value)
        or ic.print(f"Timer captured value: {captured_value}"),
    )

    repeat_id = ic.set_timer_interval(interval, repeat_timer_callback)

    single_cross_canister_id = ic.set_timer(delay, single_cross_canister_timer_callback)

    repeat_cross_canister_id = ic.set_timer_interval(
        interval, repeat_cross_canister_timer_callback
    )

    return {
        "single": single_id,
        "inline": inline_id,
        "capture": capture_id,
        "repeat": repeat_id,
        "single_cross_canister": single_cross_canister_id,
        "repeat_cross_canister": repeat_cross_canister_id,
    }


@query
def status_report() -> StatusReport:
    return status


def one_time_timer_callback():
    status["single"] = True
    ic.print("one_time_timer_callback called")


def repeat_timer_callback():
    status["repeat"] += 1
    ic.print(f"Repeating timer. Call {status['repeat']}")


def update_inline_status():
    status["inline"] = 1


def update_capture_status(value: str):
    status["capture"] = value


# TODO It would probably be better for this to have a return type of Async[void] once we have void types working
def single_cross_canister_timer_callback() -> Async[blob]:
    ic.print("single_cross_canister_timer_callback")

    result: CanisterResult[blob] = yield management_canister.raw_rand()

    if result.err is not None:
        return bytes()

    status["single_cross_canister"] = result.ok

    return result.ok


# TODO It would probably be better for this to have a return type of Async[void] once we have void types working
def repeat_cross_canister_timer_callback() -> Async[blob]:
    ic.print("repeat_cross_canister_timer_callback")

    result: CanisterResult[blob] = yield management_canister.raw_rand()

    if result.err is not None:
        return bytes()

    status["repeat_cross_canister"] += result.ok

    return result.ok
```
