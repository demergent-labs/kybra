from kybra import GuardResult, ic, int32, Manual, query, Record, update

# region Types


class State(Record):
    counter: int32
    heartbeat_tick: int32


class CustomError(Exception):
    def __init__(self, message: str):
        self.message = message


# endregion Types

state: State = {"counter": 0, "heartbeat_tick": 0}

# region GuardFunctions


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
    ic.print("throw_string called")
    raise Exception('Execution halted by "throw string" guard function')


def throw_custom_error() -> GuardResult:
    ic.print("throw_custom_error called")
    raise CustomError('Execution halted by "throw custom error" guard function')


def prevent_upgrades() -> GuardResult:
    ic.print("prevent_upgrades called")
    return {"Err": "Upgrades to this canister are disabled"}


def return_invalid_type() -> GuardResult:
    ic.print("return_invalid_type called")
    return "Something other than a guard result"  # type: ignore


def return_non_guard_result_object() -> GuardResult:
    ic.print("return_non_guard_result_object called")
    return {"badProp": "Something other than a guard result"}  # type: ignore


def return_non_null_ok_value() -> GuardResult:
    ic.print("non_null_ok_value called")
    return {"Ok": "Something other than null"}  # type: ignore


def return_non_string_err_value() -> GuardResult:
    ic.print("non_string_err_value called")
    return {"Err": {"badProp": "Something other than a string"}}  # type: ignore


def name_error() -> GuardResult:
    ic.print("name_error called")
    return {Ok: "'Ok' key should be a string, not an identifier"}  # type: ignore


# endregion GuardFunctions


@query
def get_state() -> State:
    return state


@query
def identifier_annotation() -> bool:
    ic.print("identifier_annotation called")
    return True


@query()
def call_expression_without_options_object() -> bool:
    ic.print("call_expression_without_options_object")
    return True


@query(guard=allow_all)
def loosely_guarded() -> bool:
    ic.print("loosely_guarded called")
    return True


@query(guard=allow_all)
def loosely_guarded_manual() -> Manual[bool]:
    ic.print("loosely_guarded_manual called")
    ic.reply(True)


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


# Execution halted by runtime error
@query(guard=return_invalid_type)
def invalid_return_type_guarded() -> bool:
    ic.print("invalid_return_type_guarded called")
    return True


@query(guard=return_non_guard_result_object)
def bad_object_guarded() -> bool:
    ic.print("bad_object_guarded called")
    return True


@query(guard=return_non_null_ok_value)
def non_null_ok_value_guarded() -> bool:
    ic.print("non_null_ok_value_guarded called")
    return True


@query(guard=return_non_string_err_value)
def non_string_err_value_guarded() -> bool:
    ic.print("non_string_err_value_guarded called")
    return True


@query(guard=name_error)
def name_error_guarded() -> bool:
    ic.print("name_error_guarded called")
    return True
