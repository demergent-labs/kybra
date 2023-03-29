from kybra import InsertError, null, opt, query, StableBTreeMap, update, Variant
from kybra import nat64

stable_map8 = StableBTreeMap[bool, null](
    memory_id=8, max_key_size=100, max_value_size=1_000)


class StableMap8InsertResult(Variant, total=False):
    Ok: opt[null]
    Err: InsertError


@query
def stable_map8_get(key: bool) -> opt[null]:
    return stable_map8.get(key)


@update
def stable_map8_insert(key: bool, value: null) -> StableMap8InsertResult:
    result = stable_map8.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map8_remove(key: bool) -> opt[null]:
    return stable_map8.remove(key)


@query
def stable_map8_contains_key(key: bool) -> bool:
    return stable_map8.contains_key(key)


@query
def stable_map8_is_empty() -> bool:
    return stable_map8.is_empty()


@query
def stable_map8_keys() -> list[bool]:
    return stable_map8.keys()


@query
def stable_map8_values() -> list[null]:
    return stable_map8.values()


@query
def stable_map8_items() -> list[tuple[bool, null]]:
    return stable_map8.items()


@query
def stable_map8_len() -> nat64:
    return stable_map8.len()
