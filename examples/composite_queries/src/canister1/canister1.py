from kybra import Async, CanisterResult, composite, nat, Principal, query, update
from src.canister1.types import Canister1, NatQueryResult, StringQueryResult
from src.canister2.types import Canister2


canister1 = Canister1(Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai'))
canister2 = Canister2(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))
counter: nat = 0


# Composite query calling a query
@query
@composite
def simple_composite_query() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.simple_query()

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


# Composite query calling a manual query
@composite
@query
def manual_query() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.manual_query()

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


# Composite query calling another composite query
@query
@composite
def deep_query() -> Async[StringQueryResult]:
    result: CanisterResult[StringQueryResult] = yield canister2.deep_query()

    if result.err is not None:
        return {
            'err': result.err
        }
    if 'err' in result.ok:
        return {
            'err': result.ok['err']
        }
    if 'ok' not in result.ok:
        return {
            'err': 'Unreachable'
        }

    return {
        'ok': result.ok['ok']
    }


# Composite query calling an update method. SHOULDN'T WORK
@query
@composite
def update_query() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.update_query()

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


# Composite query being called by a query method. SHOULDN'T WORK
@query
def simple_query() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.simple_query()

    if result.err is not None:
        return {
            'err': f'So is this our err {result.err}'
        }

    return {
        'ok': result.ok
    }


# Composite query being called by an update method. SHOULDN'T WORK
@update
def simple_update() -> Async[StringQueryResult]:
    result: CanisterResult[str] = yield canister2.deep_query()

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


# Composite query that modifies the state. Should revert after the call is done
@query
@composite
def inc_counter() -> nat:
    global counter
    counter += 1
    return counter


# Composite query calling queries on the same canister. SHOULDN'T WORK
@query
@composite
def inc_canister1() -> Async[NatQueryResult]:
    global counter
    counter += 1

    canister1_a: CanisterResult[nat] = yield canister1.inc_counter()
    if canister1_a.err is not None:
        return {
            'err': canister1_a.err
        }

    canister1_b: CanisterResult[nat] = yield canister1.inc_counter()
    if canister1_b.err is not None:
        return {
            'err': canister1_b.err
        }

    return {
        'ok': counter + canister1_a.ok + canister1_b.ok
    }


# Composite query calling queries that modify the state
@query
@composite
def inc_canister2() -> Async[NatQueryResult]:
    global counter
    counter += 1

    canister2_a: CanisterResult[nat] = yield canister2.inc_counter()
    if canister2_a.err is not None:
        return {
            'err': canister2_a.err
        }

    canister2_b: CanisterResult[nat] = yield canister2.inc_counter()
    if canister2_b.err is not None:
        return {
            'err': canister2_b.err
        }

    return {
        'ok': counter + canister2_a.ok + canister2_b.ok
    }
