from kybra import (
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
)
from kybra import nat, nat64, nat32


stable_map2 = StableBTreeMap[nat32, nat](
    memory_id=2, max_key_size=100, max_value_size=1_000
)


@query
def stable_map2_get(key: nat32) -> Opt[nat]:
    return stable_map2.get(key)


@update
def stable_map2_insert(key: nat32, value: nat) -> Opt[nat]:
    return stable_map2.insert(key, value)


@update
def stable_map2_remove(key: nat32) -> Opt[nat]:
    return stable_map2.remove(key)


@query
def stable_map2_contains_key(key: nat32) -> bool:
    return stable_map2.contains_key(key)


@query
def stable_map2_is_empty() -> bool:
    return stable_map2.is_empty()


@query
def stable_map2_keys() -> Vec[nat32]:
    return stable_map2.keys()


@query
def stable_map2_values() -> Vec[nat]:
    return stable_map2.values()


@query
def stable_map2_items() -> Vec[Tuple[nat32, nat]]:
    return stable_map2.items()


@query
def stable_map2_len() -> nat64:
    return stable_map2.len()
