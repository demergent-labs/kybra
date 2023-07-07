from kybra import (
    ic,
    init,
    post_upgrade,
    pre_upgrade,
    query,
    void,
)

message = ""


@query
def get_message() -> str:
    return message


@init
def init_(should_trap: bool) -> void:
    ic.print("init_")

    global message
    message = "init_"

    if should_trap:
        ic.trap("init_ trapped")


@pre_upgrade
def pre_upgrade_() -> void:
    ic.print("pre_upgrade_")

    global message
    message = "pre_upgrade_"


@post_upgrade
def post_upgrade_(should_trap: bool) -> void:
    ic.print("post_upgrade_")

    global message
    message = "post_upgrade_"

    if should_trap:
        ic.trap("post_upgrade_ trapped")
