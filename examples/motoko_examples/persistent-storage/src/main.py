from kybra import ic, init, nat, Record, query, update


class StableStorage(Record):
    counter: nat


stable_storage: StableStorage = ic.stable_storage()


@init
def init_():
    global stable_storage
    stable_storage['counter'] = 0


@update
def increment() -> nat:
    global stable_storage
    stable_storage['counter'] += 1
    return stable_storage['counter']


@query
def get() -> nat:
    return stable_storage['counter']


@update
def reset() -> nat:
    global stable_storage
    stable_storage['counter'] = 0
    return stable_storage['counter']
