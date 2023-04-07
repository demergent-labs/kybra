from kybra import InsertError, match, opt, query, StableBTreeMap, update, Vec, Variant
from kybra import nat64
from kybra import float32


class StableMap10InsertResult(Variant, total=False):
    Ok: opt[opt[bool]]
    Err: InsertError


stable_map10 = StableBTreeMap[float32, opt[bool]](
    memory_id=10, max_key_size=100, max_value_size=1_000
)


@query
def stable_map10_get(key: float32) -> opt[opt[bool]]:
    return stable_map10.get(key)


@update
def stable_map10_insert(key: float32, value: opt[bool]) -> StableMap10InsertResult:
    result = stable_map10.insert(key, value)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


@update
def stable_map10_remove(key: float32) -> opt[opt[bool]]:
    return stable_map10.remove(key)


@query
def stable_map10_contains_key(key: float32) -> bool:
    return stable_map10.contains_key(key)


@query
def stable_map10_is_empty() -> bool:
    return stable_map10.is_empty()


@query
def stable_map10_keys() -> Vec[float32]:
    return stable_map10.keys()


@query
def stable_map10_values() -> Vec[opt[bool]]:
    return stable_map10.values()


@query
def stable_map10_items() -> Vec[tuple[float32, opt[bool]]]:
    return stable_map10.items()


@query
def stable_map10_len() -> nat64:
    return stable_map10.len()
