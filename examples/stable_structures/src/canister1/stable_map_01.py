from kybra import (
    blob,
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
)
from kybra import nat64, nat16


stable_map1 = StableBTreeMap[nat16, blob](
    memory_id=1, max_key_size=100, max_value_size=1_000
)


@query
def stable_map1_get(key: nat16) -> Opt[blob]:
    return stable_map1.get(key)


@update
def stable_map1_insert(key: nat16, value: blob) -> Opt[blob]:
    return stable_map1.insert(key, value)


@update
def stable_map1_remove(key: nat16) -> Opt[blob]:
    return stable_map1.remove(key)


@query
def stable_map1_contains_key(key: nat16) -> bool:
    return stable_map1.contains_key(key)


@query
def stable_map1_is_empty() -> bool:
    return stable_map1.is_empty()


@query
def stable_map1_keys() -> Vec[nat16]:
    return stable_map1.keys()


@query
def stable_map1_values() -> Vec[blob]:
    return stable_map1.values()


@query
def stable_map1_items() -> Vec[Tuple[nat16, blob]]:
    return stable_map1.items()


@query
def stable_map1_len() -> nat64:
    return stable_map1.len()
