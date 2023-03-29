from kybra import InsertError, blob, opt, query, StableBTreeMap, update, Variant
from kybra import nat64, nat16


class StableMap1InsertResult(Variant, total=False):
    Ok: opt[blob]
    Err: InsertError


stable_map1 = StableBTreeMap[nat16, blob](
    memory_id=1, max_key_size=100, max_value_size=1_000)


@query
def stable_map1_get(key: nat16) -> opt[blob]:
    return stable_map1.get(key)


@update
def stable_map1_insert(key: nat16, value: blob) -> StableMap1InsertResult:
    result = stable_map1.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map1_remove(key: nat16) -> opt[blob]:
    return stable_map1.remove(key)


@query
def stable_map1_contains_key(key: nat16) -> bool:
    return stable_map1.contains_key(key)


@query
def stable_map1_is_empty() -> bool:
    return stable_map1.is_empty()


@query
def stable_map1_keys() -> list[nat16]:
    return stable_map1.keys()


@query
def stable_map1_values() -> list[blob]:
    return stable_map1.values()


@query
def stable_map1_items() -> list[tuple[nat16, blob]]:
    return stable_map1.items()


@query
def stable_map1_len() -> nat64:
    return stable_map1.len()
