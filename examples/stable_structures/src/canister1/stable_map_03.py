from kybra import (
    InsertError,
    nat64,
    null,
    opt,
    query,
    StableBTreeMap,
    update,
    Variant
)


class Reaction(Variant):
    Happy: null
    Sad: null


class StableMap3InsertResult(Variant, total=False):
    ok: opt[int]
    err: InsertError


stable_map_3 = StableBTreeMap[Reaction, int](
    memory_id=3, max_key_size=100, max_value_size=1_000)


@query
def stable_map_3_get(key: Reaction) -> opt[int]:
    return stable_map_3.get(key)


@update
def stable_map_3_insert(key: Reaction, value: int) -> StableMap3InsertResult:
    result = stable_map_3.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_3_remove(key: Reaction) -> opt[int]:
    return stable_map_3.remove(key)


@query
def stable_map_3_contains_key(key: Reaction) -> bool:
    return stable_map_3.contains_key(key)


@query
def stable_map_3_is_empty() -> bool:
    return stable_map_3.is_empty()


@query
def stable_map_3_keys() -> list[Reaction]:
    return stable_map_3.keys()


@query
def stable_map_3_values() -> list[int]:
    return stable_map_3.values()


@query
def stable_map_3_items() -> list[tuple[Reaction, int]]:
    return stable_map_3.items()


@query
def stable_map_3_len() -> nat64:
    return stable_map_3.len()
