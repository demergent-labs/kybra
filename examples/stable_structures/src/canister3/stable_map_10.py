from kybra import InsertError, opt, query, StableBTreeMap, update, Variant
from kybra import nat64
from kybra import float32


class StableMap10InsertResult(Variant, total=False):
    Ok: opt[opt[bool]]
    Err: InsertError


stable_map_10 = StableBTreeMap[float32, opt[bool]](
    memory_id=10, max_key_size=100, max_value_size=1_000)


@query
def stable_map_10_get(key: float32) -> opt[opt[bool]]:
    return stable_map_10.get(key)


@update
def stable_map_10_insert(key: float32, value: opt[bool]) -> StableMap10InsertResult:
    result = stable_map_10.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map_10_remove(key: float32) -> opt[opt[bool]]:
    return stable_map_10.remove(key)


@query
def stable_map_10_contains_key(key: float32) -> bool:
    return stable_map_10.contains_key(key)


@query
def stable_map_10_is_empty() -> bool:
    return stable_map_10.is_empty()


@query
def stable_map_10_keys() -> list[float32]:
    return stable_map_10.keys()


@query
def stable_map_10_values() -> list[opt[bool]]:
    return stable_map_10.values()


@query
def stable_map_10_items() -> list[tuple[float32, opt[bool]]]:
    return stable_map_10.items()


@query
def stable_map_10_len() -> nat64:
    return stable_map_10.len()
