from kybra import Duration, ic, nat8, query, Record, TimerId, update


class StatusReport(Record):
    single: bool
    repeat: nat8


class TimerIds(Record):
    single: TimerId
    repeat: TimerId


status: StatusReport = {"single": False, "repeat": 0}


@update
def clear_timer(timer_id: TimerId):
    ic.clear_timer(timer_id)
    ic.print(f"timer {timer_id} cancelled")


@update
def set_timers(delay: Duration, interval: Duration) -> TimerIds:
    single_id = ic.set_timer(delay, one_time_timer_callback)
    ic.print(f"Set timer {single_id}")

    repeat_id = ic.set_timer_interval(interval, repeat_timer_callback)
    ic.print(f"Set timer {single_id}")

    return {
        "single": single_id,
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
