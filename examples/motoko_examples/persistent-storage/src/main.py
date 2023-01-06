from kybra import init, nat, query, StableBTreeMap, update

stable_storage = StableBTreeMap[str, nat](memory_id=0, max_key_size=15, max_value_size=1_000)


@init
def init_():
    stable_storage.insert('counter', 0)


@update
def increment() -> nat:
    counter = stable_storage.get('counter') or 0
    new_counter = counter + 1

    stable_storage.insert('counter', new_counter)

    return new_counter
    # return stable_storage.insert('counter', counter + 1) TODO do this once the insert return type is fixed


@query
def get() -> nat:
    return stable_storage.get('counter') or 0


@update
def reset() -> nat:
    return 0
    # return stable_storage.insert('counter', 0) TODO do this once the insert return type is fixed
