from kybra import nat, query, update, void

counter: nat = 0


@query
def get() -> nat:
    return counter


@update
def set(n: nat) -> void:
    global counter

    counter = n


@update
def inc() -> void:
    global counter

    counter += 1
