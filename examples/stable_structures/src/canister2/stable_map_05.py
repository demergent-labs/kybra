from kybra import InsertError, opt, query, StableBTreeMap, update, Variant
from kybra import nat64
from kybra import float64


class StableMap5InsertResult(Variant, total=False):
    Ok: opt[float64]
    Err: InsertError


stable_map5 = StableBTreeMap[opt[str], float64](
    memory_id=5, max_key_size=100, max_value_size=1_000)


@query
def stable_map5_get(key: opt[str]) -> opt[float64]:
    return stable_map5.get(key)


@update
def stable_map5_insert(key: opt[str], value: float64) -> StableMap5InsertResult:
    result = stable_map5.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map5_remove(key: opt[str]) -> opt[float64]:
    return stable_map5.remove(key)


@query
def stable_map5_contains_key(key: opt[str]) -> bool:
    return stable_map5.contains_key(key)


@query
def stable_map5_is_empty() -> bool:
    return stable_map5.is_empty()


@query
def stable_map5_keys() -> list[opt[str]]:
    return stable_map5.keys()


@query
def stable_map5_values() -> list[float64]:
    return stable_map5.values()


@query
def stable_map5_items() -> list[tuple[opt[str], float64]]:
    return stable_map5.items()


@query
def stable_map5_len() -> nat64:
    return stable_map5.len()
