from kybra import (
    nat64,
    null,
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Variant,
    Vec,
)


class Reaction(Variant):
    Happy: null
    Sad: null


stable_map3 = StableBTreeMap[Reaction, int](
    memory_id=3, max_key_size=100, max_value_size=1_000
)


@query
def stable_map3_get(key: Reaction) -> Opt[int]:
    return stable_map3.get(key)


@update
def stable_map3_insert(key: Reaction, value: int) -> Opt[int]:
    return stable_map3.insert(key, value)


@update
def stable_map3_remove(key: Reaction) -> Opt[int]:
    return stable_map3.remove(key)


@query
def stable_map3_contains_key(key: Reaction) -> bool:
    return stable_map3.contains_key(key)


@query
def stable_map3_is_empty() -> bool:
    return stable_map3.is_empty()


@query
def stable_map3_keys() -> Vec[Reaction]:
    return stable_map3.keys()


@query
def stable_map3_values() -> Vec[int]:
    return stable_map3.values()


@query
def stable_map3_items() -> Vec[Tuple[Reaction, int]]:
    return stable_map3.items()


@query
def stable_map3_len() -> nat64:
    return stable_map3.len()
