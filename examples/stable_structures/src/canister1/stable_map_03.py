from kybra import (
    InsertError,
    match,
    nat64,
    null,
    opt,
    query,
    StableBTreeMap,
    update,
    Variant,
)


class Reaction(Variant):
    Happy: null
    Sad: null


class StableMap3InsertResult(Variant, total=False):
    Ok: opt[int]
    Err: InsertError


stable_map3 = StableBTreeMap[Reaction, int](
    memory_id=3, max_key_size=100, max_value_size=1_000
)


@query
def stable_map3_get(key: Reaction) -> opt[int]:
    return stable_map3.get(key)


@update
def stable_map3_insert(key: Reaction, value: int) -> StableMap3InsertResult:
    result = stable_map3.insert(key, value)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


@update
def stable_map3_remove(key: Reaction) -> opt[int]:
    return stable_map3.remove(key)


@query
def stable_map3_contains_key(key: Reaction) -> bool:
    return stable_map3.contains_key(key)


@query
def stable_map3_is_empty() -> bool:
    return stable_map3.is_empty()


@query
def stable_map3_keys() -> list[Reaction]:
    return stable_map3.keys()


@query
def stable_map3_values() -> list[int]:
    return stable_map3.values()


@query
def stable_map3_items() -> list[tuple[Reaction, int]]:
    return stable_map3.items()


@query
def stable_map3_len() -> nat64:
    return stable_map3.len()
