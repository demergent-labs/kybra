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

stable_map8 = StableBTreeMap[bool, null](
    memory_id=8, max_key_size=100, max_value_size=1_000
)


@query
def stable_map8_get(key: bool) -> Opt[null]:
    return stable_map8.get(key)


@update
def stable_map8_insert(key: bool, value: null) -> Opt[null]:
    return stable_map8.insert(key, value)


@update
def stable_map8_remove(key: bool) -> Opt[null]:
    return stable_map8.remove(key)


@query
def stable_map8_contains_key(key: bool) -> bool:
    return stable_map8.contains_key(key)


@query
def stable_map8_is_empty() -> bool:
    return stable_map8.is_empty()


@query
def stable_map8_keys() -> Vec[bool]:
    return stable_map8.keys()


@query
def stable_map8_values() -> Vec[null]:
    return stable_map8.values()


@query
def stable_map8_items() -> Vec[Tuple[bool, null]]:
    return stable_map8.items()


@query
def stable_map8_len() -> nat64:
    return stable_map8.len()
