from kybra import Async, CanisterResult, nat, Principal, query, update, manual, ic
from src.canister1.types import Canister1, NatQueryResult, StringQueryResult
from src.canister2.types import Canister2


canister1 = Canister1(Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai'))
canister2 = Canister2(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))
counter: nat = 0


# Composite query calling a query
@query
def simple_composite_query() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.simple_query()

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


# Composite query calling a manual query
@query
def manual_query() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.manual_query()

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


# Manual composite query calling a manual query
@query
def totally_manual_query() -> Async[manual[StringQueryResult]]:
    result: CanisterResult[str] = yield canister2.manual_query()
    # ic.reply(result)

    if result.Err is not None:
        ic.reply({
            'Err': result.Err
        })

    ic.reply({
        'Ok': result.Ok
    })


# Composite query calling another composite query
@query
def deep_query() -> Async[StringQueryResult]:
    result: CanisterResult[StringQueryResult] = yield canister2.deep_query()

    if result.Err is not None:
        return {
            'Err': result.Err
        }
    if 'Err' in result.Ok:
        return {
            'Err': result.Ok['Err']
        }
    if 'Ok' not in result.Ok:
        return {
            'Err': 'Unreachable'
        }

    return {
        'Ok': result.Ok['Ok']
    }


# Composite query calling an update method. SHOULDN'T WORK
@query
def update_query() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.update_query()

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


# Composite query being called by an update method. SHOULDN'T WORK
@update
def simple_update() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.deep_query()

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


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

    canister1_a_result: CanisterResult[nat] = yield canister1.inc_counter()
    if canister1_a_result.Err is not None:
        return {
            'Err': canister1_a_result.Err
        }

    canister1_b_result: CanisterResult[nat] = yield canister1.inc_counter()
    if canister1_b_result.Err is not None:
        return {
            'Err': canister1_b_result.Err
        }

    return {
        'Ok': counter + canister1_a_result .Ok + canister1_b_result.Ok
    }


# Composite query calling queries that modify the state
@query
def inc_canister2() -> Async[NatQueryResult]:
    global counter
    counter += 1

    canister2_a_result: CanisterResult[nat] = yield canister2.inc_counter()
    if canister2_a_result.Err is not None:
        return {
            'Err': canister2_a_result.Err
        }

    canister2_b_result: CanisterResult[nat] = yield canister2.inc_counter()
    if canister2_b_result.Err is not None:
        return {
            'Err': canister2_b_result.Err
        }

    return {
        'Ok': counter + canister2_a_result .Ok + canister2_b_result.Ok
    }
