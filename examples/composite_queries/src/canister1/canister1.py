from kybra import (
    Async,
    CallResult,
    match,
    nat,
    Principal,
    query,
    update,
    Manual,
    ic,
)
from src.canister1.types import Canister1, NatQueryResult, StringQueryResult
from src.canister2.types import Canister2


canister1 = Canister1(Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai"))
canister2 = Canister2(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))
counter: nat = 0


# Composite query calling a query
@query
def simple_composite_query() -> Async[StringQueryResult]:
    result: CallResult[str] = yield canister2.simple_query()

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


# Composite query calling a manual query
@query
def manual_query() -> Async[StringQueryResult]:
    result: CallResult[str] = yield canister2.manual_query()

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


# Manual composite query calling a manual query
@query
def totally_manual_query() -> Async[Manual[StringQueryResult]]:
    result: CallResult[str] = yield canister2.manual_query()
    # ic.reply(result)

    match(
        result,
        {
            "Ok": lambda ok: ic.reply({"Ok": ok}),
            "Err": lambda err: ic.reply({"Err": err}),
        },
    )


# Composite query calling another composite query
@query
def deep_query() -> Async[StringQueryResult]:
    result: CallResult[StringQueryResult] = yield canister2.deep_query()

    return match(
        result,
        {
            "Ok": lambda string_query_result: match(
                string_query_result,
                {
                    "Ok": lambda string_query: {"Ok": string_query},
                    "Err": lambda err: {"Err": err},
                },
            ),
            "Err": lambda err: {"Err": err},
        },
    )


# Composite query calling an update method. SHOULDN'T WORK
@query
def update_query() -> Async[StringQueryResult]:
    result: CallResult[str] = yield canister2.update_query()

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


# Composite query being called by an update method. SHOULDN'T WORK
@update
def simple_update() -> Async[StringQueryResult]:
    result: CallResult[str] = yield canister2.deep_query()

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


# Composite query that modifies the state. Should revert after the call is done
@query
def inc_counter() -> nat:
    global counter
    counter += 1
    return counter


# Composite query calling queries on the same canister. SHOULDN'T WORK
@query
def inc_canister1() -> Async[NatQueryResult]:
    global counter
    counter += 1

    canister1_a_result: CallResult[nat] = yield canister1.inc_counter()
    canister1_b_result: CallResult[nat] = (yield canister1.inc_counter())

    return match(
        canister1_a_result,
        {
            "Ok": lambda canister1_a_ok: match(
                canister1_b_result,
                {
                    "Ok": lambda canister1_b_ok: {
                        "Ok": counter + canister1_a_ok + canister1_b_ok
                    },
                    "Err": lambda err: {"Err": err},
                },
            ),
            "Err": lambda err: {"Err": err},
        },
    )


# Composite query calling queries that modify the state
@query
def inc_canister2() -> Async[NatQueryResult]:
    global counter
    counter += 1

    canister2_a_result: CallResult[nat] = yield canister2.inc_counter()
    canister2_b_result: CallResult[nat] = (yield canister2.inc_counter())

    return match(
        canister2_a_result,
        {
            "Ok": lambda canister2_a_ok: match(
                canister2_b_result,
                {
                    "Ok": lambda canister2_b_ok: {
                        "Ok": counter + canister2_a_ok + canister2_b_ok
                    },
                    "Err": lambda err: {"Err": err},
                },
            ),
            "Err": lambda err: {"Err": err},
        },
    )
