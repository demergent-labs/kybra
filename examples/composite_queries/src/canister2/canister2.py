from kybra import Async, CanisterResult, composite, ic, manual, nat, Principal, query, update
from src.canister1.types import StringQueryResult
from src.canister3.types import Canister3

canister3 = Canister3(Principal.from_str('r7inp-6aaaa-aaaaa-aaabq-cai'))

counter: nat = 0


@query
@composite
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
def manual_query() -> manual[str]:
    ic.reply("Hello from Canister 2 manual_query")


@query
@composite
def deep_query() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister3.deep_query()

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }
