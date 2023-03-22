from kybra import InsertError, blob, null, opt, query, StableBTreeMap, update, Variant
from kybra import nat64


class Reaction(Variant):
    Happy: null
    Sad: null


class StableMap12InsertResult(Variant, total=False):
    Ok: opt[Reaction]
    Err: InsertError


stable_map_12 = StableBTreeMap[blob, Reaction](
    memory_id=12, max_key_size=100, max_value_size=1_000)


@query
def stable_map_12_get(key: blob) -> opt[Reaction]:
    return stable_map_12.get(key)


@update
def stable_map_12_insert(key: blob, value: Reaction) -> StableMap12InsertResult:
    result = stable_map_12.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map_12_remove(key: blob) -> opt[Reaction]:
    return stable_map_12.remove(key)


@query
def stable_map_12_contains_key(key: blob) -> bool:
    return stable_map_12.contains_key(key)


@query
def stable_map_12_is_empty() -> bool:
    return stable_map_12.is_empty()


@query
def stable_map_12_keys() -> list[blob]:
    return stable_map_12.keys()


@query
def stable_map_12_values() -> list[Reaction]:
    return stable_map_12.values()


@query
def stable_map_12_items() -> list[tuple[blob, Reaction]]:
    return stable_map_12.items()


@query
def stable_map_12_len() -> nat64:
    return stable_map_12.len()
