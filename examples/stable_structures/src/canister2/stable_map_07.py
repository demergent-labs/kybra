from kybra import InsertError, null, opt, query, StableBTreeMap, update, Variant
from kybra import nat64


class StableMap7InsertResult(Variant, total=False):
    Ok: opt[null]
    Err: InsertError


stable_map7 = StableBTreeMap[null, null](
    memory_id=7, max_key_size=100, max_value_size=1_000)


@query
def stable_map7_get(key: null) -> opt[null]:
    return stable_map7.get(key)


@update
def stable_map7_insert(key: null, value: null) -> StableMap7InsertResult:
    result = stable_map7.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map7_remove(key: null) -> opt[null]:
    return stable_map7.remove(key)


@query
def stable_map7_contains_key(key: null) -> bool:
    return stable_map7.contains_key(key)


@query
def stable_map7_is_empty() -> bool:
    return stable_map7.is_empty()


@query
def stable_map7_keys() -> list[null]:
    return stable_map7.keys()


@query
def stable_map7_values() -> list[null]:
    return stable_map7.values()


@query
def stable_map7_items() -> list[tuple[null, null]]:
    return stable_map7.items()


@query
def stable_map7_len() -> nat64:
    return stable_map7.len()
