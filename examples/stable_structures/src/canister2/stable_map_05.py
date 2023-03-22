from kybra import InsertError, opt, query, StableBTreeMap, update, Variant
from kybra import nat64
from kybra import float64


class StableMap5InsertResult(Variant, total=False):
    Ok: opt[float64]
    Err: InsertError


stable_map_5 = StableBTreeMap[opt[str], float64](
    memory_id=5, max_key_size=100, max_value_size=1_000)


@query
def stable_map_5_get(key: opt[str]) -> opt[float64]:
    return stable_map_5.get(key)


@update
def stable_map_5_insert(key: opt[str], value: float64) -> StableMap5InsertResult:
    result = stable_map_5.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map_5_remove(key: opt[str]) -> opt[float64]:
    return stable_map_5.remove(key)


@query
def stable_map_5_contains_key(key: opt[str]) -> bool:
    return stable_map_5.contains_key(key)


@query
def stable_map_5_is_empty() -> bool:
    return stable_map_5.is_empty()


@query
def stable_map_5_keys() -> list[opt[str]]:
    return stable_map_5.keys()


@query
def stable_map_5_values() -> list[float64]:
    return stable_map_5.values()


@query
def stable_map_5_items() -> list[tuple[opt[str], float64]]:
    return stable_map_5.items()


@query
def stable_map_5_len() -> nat64:
    return stable_map_5.len()
