from kybra import Duration, ic, nat8, query, Record, TimerId, update


class StatusReport(Record):
    single: bool
    inline1: nat8
    inline2: nat8
    inner: str
    repeat: nat8


class TimerIds(Record):
    single: TimerId
    inline1: TimerId
    inline2: TimerId
    inner: TimerId
    repeat: TimerId


status: StatusReport = {
    "single": False,
    "inline1": 0,
    "inline2": 0,
    "inner": "",
    "repeat": 0,
}


@update
def clear_timer(timer_id: TimerId):
    ic.clear_timer(timer_id)
    ic.print(f"timer {timer_id} cancelled")


@update
def set_timers(delay: Duration, interval: Duration) -> TimerIds:
    captured_value = "🚩"

    def inner_closure():
        global status
        status["inner"] = captured_value
        ic.print(f"Inner function captured value: {captured_value}")

    single_id = ic.set_timer(delay, one_time_timer_callback)
    ic.print(f"Set timer {single_id}")

    # Note: Demergent Labs does not recommend using lambdas in timers. They will
    # be called, but they are limited in functionality. Calling a defined
    # function is preferred whenever possible.

    # Note: You cannot set global variables from within a lambda but you can
    # call a function that sets a global variable. So we've moved the "setting"
    # functionality out into helper functions while the printing is kept here in
    # the lambda.

    inline1_id = ic.set_timer(
        delay,
        lambda: update_status_report_inline_1() or ic.print("Inline timer 1 called"),
    )
    ic.print(f"Set timer {inline1_id}")

    inline2_id = ic.set_timer(
        delay,
        lambda: update_status_report_inline_2() or ic.print("Inline timer 2 called"),
    )
    ic.print(f"Set timer {inline2_id}")

    inner_id = ic.set_timer(delay, inner_closure)
    ic.print(f"Set timer {single_id}")

    repeat_id = ic.set_timer_interval(interval, repeat_timer_callback)
    ic.print(f"Set timer {repeat_id}")

    return {
        "single": single_id,
        "inline1": inline1_id,
        "inline2": inline2_id,
        "inner": inner_id,
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


def update_status_report_inline_1():
    global status
    status["inline1"] = 1


def update_status_report_inline_2():
    global status
    status["inline2"] = 2
