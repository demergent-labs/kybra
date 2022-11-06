from kybra import ic, nat, nat64, query, update

@update
def receive_cycles() -> nat64:
    return ic.msg_cycles_accept(ic.msg_cycles_available() // 2)

@update
def receive_cycles128() -> nat:
    return ic.msg_cycles_accept128(ic.msg_cycles_available128() // 2)

@query
def get_canister_balance() -> nat64:
    return ic.canister_balance()

@query
def get_canister_balance128() -> nat:
    return ic.canister_balance128()
