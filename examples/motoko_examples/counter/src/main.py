from kybra import nat, query, update

counter: nat = 0


@query
def get() -> nat:
    return counter


@update
def set(n: nat):
    global counter

    counter = n


@update
def inc():
    global counter

    counter += 1
