from kybra import ic, inspect_message, update, void


@inspect_message
def inspect_message_() -> void:
    ic.print("inspect_message called")

    if (
        ic.method_name() == "accessible"
        or ic.method_name() == "does_interpreter_exist"
        or ic.method_name() == "__get_candid_interface_tmp_hack"
    ):
        ic.accept_message()
        return

    if ic.method_name() == "inaccessible":
        return

    raise Exception("Method " + ic.method_name() + " is not allowed")


@update
def accessible() -> bool:
    return True


@update
def inaccessible() -> bool:
    return False


@update
def also_inaccessible() -> bool:
    return False
