from kybra import nat, query, update

counter: nat = 0


@update
def count() -> nat:
    global counter
    counter += 1
    return counter


@query
def get_count() -> nat:
    return counter


@update
def reset() -> nat:
    global counter
    counter = 0
    return counter
