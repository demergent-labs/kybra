from kybra import (
    InsertError,
    match,
    nat64,
    nat8,
    opt,
    query,
    StableBTreeMap,
    update,
    Variant,
)


class StableMap0InsertResult(Variant, total=False):
    Ok: opt[str]
    Err: InsertError


stable_map0 = StableBTreeMap[nat8, str](
    memory_id=0, max_key_size=100, max_value_size=100
)


@query
def stable_map0_get(key: nat8) -> opt[str]:
    return stable_map0.get(key)


@update
def stable_map0_insert(key: nat8, value: str) -> StableMap0InsertResult:
    result = stable_map0.insert(key, value)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


@update
def stable_map0_remove(key: nat8) -> opt[str]:
    return stable_map0.remove(key)


@query
def stable_map0_contains_key(key: nat8) -> bool:
    return stable_map0.contains_key(key)


@query
def stable_map0_is_empty() -> bool:
    return stable_map0.is_empty()


@query
def stable_map0_keys() -> list[nat8]:
    return stable_map0.keys()


@query
def stable_map0_values() -> list[str]:
    return stable_map0.values()


@query
def stable_map0_items() -> list[tuple[nat8, str]]:
    return stable_map0.items()


@query
def stable_map0_len() -> nat64:
    return stable_map0.len()
