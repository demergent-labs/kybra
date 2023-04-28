from kybra import (
    null,
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
)
from kybra import nat64


stable_map7 = StableBTreeMap[null, null](
    memory_id=7, max_key_size=100, max_value_size=1_000
)


@query
def stable_map7_get(key: null) -> Opt[null]:
    return stable_map7.get(key)


@update
def stable_map7_insert(key: null, value: null) -> Opt[null]:
    return stable_map7.insert(key, value)


@update
def stable_map7_remove(key: null) -> Opt[null]:
    return stable_map7.remove(key)


@query
def stable_map7_contains_key(key: null) -> bool:
    return stable_map7.contains_key(key)


@query
def stable_map7_is_empty() -> bool:
    return stable_map7.is_empty()


@query
def stable_map7_keys() -> Vec[null]:
    return stable_map7.keys()


@query
def stable_map7_values() -> Vec[null]:
    return stable_map7.values()


@query
def stable_map7_items() -> Vec[Tuple[null, null]]:
    return stable_map7.items()


@query
def stable_map7_len() -> nat64:
    return stable_map7.len()
