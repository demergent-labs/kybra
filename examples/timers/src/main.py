from kybra import Duration, ic, nat8, query, Record, TimerId, update


class StatusReport(Record):
    single: bool
    inline: nat8
    capture: str
    repeat: nat8


class TimerIds(Record):
    single: TimerId
    inline: TimerId
    capture: TimerId
    repeat: TimerId


status: StatusReport = {
    "single": False,
    "inline": 0,
    "capture": "",
    "repeat": 0,
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

    return {
        "single": single_id,
        "inline": inline_id,
        "capture": capture_id,
        "repeat": repeat_id,
    }


@query
def status_report() -> StatusReport:
    return status


def one_time_timer_callback():
    global status
    status["single"] = True
    ic.print("one_time_timer_callback called")


def repeat_timer_callback():
    global status
    status["repeat"] += 1
    ic.print(f"Repeating timer. Call {status['repeat']}")


def update_inline_status():
    global status
    status["inline"] = 1


def update_capture_status(value: str):
    global status
    status["capture"] = value
