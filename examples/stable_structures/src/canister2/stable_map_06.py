from kybra import (
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
)
from kybra import nat64


stable_map6 = StableBTreeMap[Vec[nat64], bool](
    memory_id=6, max_key_size=100, max_value_size=1_000
)


@query
def stable_map6_get(key: Vec[nat64]) -> Opt[bool]:
    return stable_map6.get(key)


@update
def stable_map6_insert(key: Vec[nat64], value: bool) -> Opt[bool]:
    return stable_map6.insert(key, value)


@update
def stable_map6_remove(key: Vec[nat64]) -> Opt[bool]:
    return stable_map6.remove(key)


@query
def stable_map6_contains_key(key: Vec[nat64]) -> bool:
    return stable_map6.contains_key(key)


@query
def stable_map6_is_empty() -> bool:
    return stable_map6.is_empty()


@query
def stable_map6_keys() -> Vec[Vec[nat64]]:
    return stable_map6.keys()


@query
def stable_map6_values() -> Vec[bool]:
    return stable_map6.values()


@query
def stable_map6_items() -> Vec[Tuple[Vec[nat64], bool]]:
    return stable_map6.items()


@query
def stable_map6_len() -> nat64:
    return stable_map6.len()
