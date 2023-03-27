from kybra import InsertError, opt, query, StableBTreeMap, update, Variant
from kybra import nat64


class StableMap6InsertResult(Variant, total=False):
    Ok: opt[bool]
    Err: InsertError


stable_map6 = StableBTreeMap[list[nat64], bool](
    memory_id=6, max_key_size=100, max_value_size=1_000)


@query
def stable_map6_get(key: list[nat64]) -> opt[bool]:
    return stable_map6.get(key)


@update
def stable_map6_insert(key: list[nat64], value: bool) -> StableMap6InsertResult:
    result = stable_map6.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map6_remove(key: list[nat64]) -> opt[bool]:
    return stable_map6.remove(key)


@query
def stable_map6_contains_key(key: list[nat64]) -> bool:
    return stable_map6.contains_key(key)


@query
def stable_map6_is_empty() -> bool:
    return stable_map6.is_empty()


@query
def stable_map6_keys() -> list[list[nat64]]:
    return stable_map6.keys()


@query
def stable_map6_values() -> list[bool]:
    return stable_map6.values()


@query
def stable_map6_items() -> list[tuple[list[nat64], bool]]:
    return stable_map6.items()


@query
def stable_map6_len() -> nat64:
    return stable_map6.len()
