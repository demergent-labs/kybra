from kybra import (
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
)
from kybra import nat64
from kybra import float64


stable_map5 = StableBTreeMap[Opt[str], float64](
    memory_id=5, max_key_size=100, max_value_size=1_000
)


@query
def stable_map5_get(key: Opt[str]) -> Opt[float64]:
    return stable_map5.get(key)


@update
def stable_map5_insert(key: Opt[str], value: float64) -> Opt[float64]:
    return stable_map5.insert(key, value)


@update
def stable_map5_remove(key: Opt[str]) -> Opt[float64]:
    return stable_map5.remove(key)


@query
def stable_map5_contains_key(key: Opt[str]) -> bool:
    return stable_map5.contains_key(key)


@query
def stable_map5_is_empty() -> bool:
    return stable_map5.is_empty()


@query
def stable_map5_keys() -> Vec[Opt[str]]:
    return stable_map5.keys()


@query
def stable_map5_values() -> Vec[float64]:
    return stable_map5.values()


@query
def stable_map5_items() -> Vec[Tuple[Opt[str], float64]]:
    return stable_map5.items()


@query
def stable_map5_len() -> nat64:
    return stable_map5.len()
