from kybra import Duration, ic, nat8, query, Record, TimerId, update


class StatusReport(Record):
    single: bool
    inline1: nat8
    inline2: nat8
    repeat: nat8


class TimerIds(Record):
    single: TimerId
    inline1: TimerId
    inline2: TimerId
    repeat: TimerId


status: StatusReport = {"single": False, "inline1": 0, "inline2": 0, "repeat": 0}


@update
def clear_timer(timer_id: TimerId):
    ic.clear_timer(timer_id)
    ic.print(f"timer {timer_id} cancelled")


@update
def set_timers(delay: Duration, interval: Duration) -> TimerIds:
    single_id = ic.set_timer(delay, one_time_timer_callback)

    # Note: Demergent Labs does not recommend using lambdas in timers. They will
    # be called, but they are limited in functionality. Calling a defined
    # function is preferred whenever possible.

    # Note: You cannot set global variables from within a lambda but you can
    # call a function that sets a global variable. So we've moved the "setting"
    # functionality out into helper functions while the printing is kept here in
    # the lambda.

    # inline1_id = ic.set_timer(
    #     delay, lambda: update_status_report_inline_1() or print("Inline timer 1 called")
    # )
    # inline2_id = ic.set_timer(
    #     delay, lambda: update_status_report_inline_1() or print("Inline timer 2 called")
    # )
    # repeat_id = ic.set_timer_interval(interval, repeat_timer_callback)

    return {
        "single": single_id,
        "inline1": 0,
        "inline2": 0,
        "repeat": 0,
    }


@query
def status_report() -> StatusReport:
    return status


def one_time_timer_callback():
    global status
    status["single"] = True
    print("one_time_timer_callback called")


def repeat_timer_callback():
    global status
    status["repeat"] += 1
    print(f"Repeating timer. Call ${status['repeat']}")


def update_status_report_inline_1():
    global status
    status["inline1"] = 1


def update_status_report_inline_2():
    global status
    status["inline1"] = 2
