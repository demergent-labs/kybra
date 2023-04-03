from kybra import (
    GuardResult,
    heartbeat,
    ic,
    inspect_message,
    int32,
    manual,
    pre_upgrade,
    query,
    Record,
    update,
    void,
)


def adelante() -> GuardResult:
    ic.print("We are in the adelante")
    return {
        "Ok": None,
    }


class State(Record):
    counter: int32
    heartbeat_tick: int32


state: State = {"counter": 0, "heartbeat_tick": 0}


def allow_modify_state_guarded() -> GuardResult:
    ic.print("allow_modify_state_guarded called")

    if (
        ic.method_name() == "modify_state_guarded"
        or ic.method_name() == "modifyStateGuarded"
    ):
        ic.print(
            f"Method {ic.method_name()} allowed by inspectMessage's guard function: allow_modify_state_guarded"
        )
    else:
        ic.print(
            f"Method {ic.method_name()} would be rejected by inspectMessage's guard function... but we are in inspect message mode so doing so would be a contract violation. Therefore, proceeding."
        )

    return {"Ok": None}


def allow_all() -> GuardResult:
    ic.print("allow_all called")
    return {"Ok": None}


def accept_all_then_reject_all() -> GuardResult:
    global state
    # ic.print("accept_all_then_reject called")
    state["heartbeat_tick"] += 1
    if state["heartbeat_tick"] > 20:
        # ic.print("Heartbeat suppressed")
        return {"Err": "This error message will never be seen"}
    # ic.print(f"Accepted heartbeat tick {state['heartbeat_tick']}")
    return {"Ok": None}


def increment_counter_and_allow_all() -> GuardResult:
    global state
    ic.print("incrementCounterAndAllowAll called")
    state["counter"] += 1
    return {"Ok": None}


def unpassable() -> GuardResult:
    ic.print("unpassable called")
    return {"Err": 'Execution halted by "unpassable" guard function'}


def throw_string() -> GuardResult:
    ic.print("throwString called")
    raise Exception('Execution halted by "throw string" guard function')


class CustomError(Exception):
    def __init__(self, message: str):
        self.message = message


def throw_custom_error() -> GuardResult:
    ic.print("throwCustomError called")
    raise CustomError('Execution halted by "throw custom error" guard function')


def prevent_upgrades() -> GuardResult:
    ic.print("preventUpgrades called")
    return {"Err": "Upgrades to this canister are disabled"}


@query
def get_state() -> State:
    return state


# Guarded functions are called
@inspect_message(guard=allow_modify_state_guarded)
def inspect_message_() -> void:
    ic.print("inspect message called")

    if (
        ic.method_name() == "modify_state_guarded"
        or ic.method_name() == "modifyStateGuarded"
    ):
        ic.print(f"Method {ic.method_name()} allowed by inspect_message")
        ic.accept_message()
    else:
        ic.print(f"Method {ic.method_name()} rejected by inspect_message")


@heartbeat(guard=accept_all_then_reject_all)
def heartbeat_() -> void:
    # ic.print("heartbeat called")
    pass


@pre_upgrade(guard=prevent_upgrades)
def pre_upgrade_() -> void:
    ic.print("pre_upgrade called")


@query
def identifier_annotation() -> bool:
    ic.print("identifier_annotation called")
    return True


@query()
def call_expression_without_options_object() -> bool:
    ic.print("call_expression_without_options_object")
    return True


@query
def call_expression_with_empty_options_object() -> bool:
    ic.print("call_expression_with_empty_option_object called")
    return True


@query(guard=allow_all)
def loosely_guarded() -> bool:
    ic.print("loosely_guarded called")
    return True


@update(guard=increment_counter_and_allow_all)
def modify_state_guarded() -> bool:
    ic.print("modify_state_guarded called")
    return True


@update(guard=increment_counter_and_allow_all)
def unallowed_method() -> bool:
    ic.print("modify_state_guarded called")
    return True


# Execution halted by guard function
@query(guard=unpassable)
def tightly_guarded() -> bool:
    ic.print("tightly_guarded called")
    return True


@query(guard=throw_string)
def error_string_guarded() -> bool:
    ic.print("error_string_guarded called")
    return True


@query(guard=throw_custom_error)
def custom_error_guarded() -> bool:
    ic.print("custom_error_guarded called")
    return True
