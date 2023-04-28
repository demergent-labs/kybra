from kybra import (
    Opt,
    Principal,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
)
from kybra import nat64


stable_map13 = StableBTreeMap[str, Principal](
    memory_id=13, max_key_size=100, max_value_size=1_000
)


@query
def stable_map13_get(key: str) -> Opt[Principal]:
    return stable_map13.get(key)


@update
def stable_map13_insert(key: str, value: Principal) -> Opt[Principal]:
    return stable_map13.insert(key, value)


@update
def stable_map13_remove(key: str) -> Opt[Principal]:
    return stable_map13.remove(key)


@query
def stable_map13_contains_key(key: str) -> bool:
    return stable_map13.contains_key(key)


@query
def stable_map13_is_empty() -> bool:
    return stable_map13.is_empty()


@query
def stable_map13_keys() -> Vec[str]:
    return stable_map13.keys()


@query
def stable_map13_values() -> Vec[Principal]:
    return stable_map13.values()


@query
def stable_map13_items() -> Vec[Tuple[str, Principal]]:
    return stable_map13.items()


@query
def stable_map13_len() -> nat64:
    return stable_map13.len()
