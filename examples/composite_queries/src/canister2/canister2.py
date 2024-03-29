from kybra import (
    Async,
    CallResult,
    ic,
    init,
    Manual,
    match,
    nat,
    Principal,
    query,
    update,
    void,
)
from src.canister1.types import StringQueryResult
from src.canister3.types import Canister3

canister3 = Canister3(Principal.from_str("aaaaa-aa"))

counter: nat = 0


@init
def init_(canister3_id: Principal) -> void:
    global canister3

    canister3 = Canister3(canister3_id)


@query
def inc_counter() -> nat:
    global counter
    counter += 1
    return counter


@query
def simple_query() -> str:
    return "Hello from Canister 2"


@update
def update_query() -> str:
    return "Hello from a Canister 2 update"


@query
def manual_query() -> Manual[str]:
    ic.reply("Hello from Canister 2 manual query")


@query
def deep_query() -> Async[StringQueryResult]:
    result: CallResult[str] = yield canister3.deep_query()

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})
