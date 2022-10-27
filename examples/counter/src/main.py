from kybra import nat64, query, update

count: nat64 = 0

@query
def read_count() -> nat64:
    return count

@update
def increment_count() -> nat64:
    global count

    count += 1

    return count
