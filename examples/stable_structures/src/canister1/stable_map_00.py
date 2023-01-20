from kybra import (
    InsertError,
    nat64,
    nat8,
    opt,
    query,
    StableBTreeMap,
    update,
    Variant
)


class StableMap0InsertResult(Variant, total=False):
    ok: opt[str]
    err: InsertError


stable_map_0 = StableBTreeMap[nat8, str](
    memory_id=0, max_key_size=100, max_value_size=100)


@query
def stable_map_0_get(key: nat8) -> opt[str]:
    return stable_map_0.get(key)


@update
def stable_map_0_insert(key: nat8, value: str) -> StableMap0InsertResult:
    result = stable_map_0.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_0_remove(key: nat8) -> opt[str]:
    return stable_map_0.remove(key)


@query
def stable_map_0_contains_key(key: nat8) -> bool:
    return stable_map_0.contains_key(key)


@query
def stable_map_0_is_empty() -> bool:
    return stable_map_0.is_empty()


@query
def stable_map_0_keys() -> list[nat8]:
    return stable_map_0.keys()


@query
def stable_map_0_values() -> list[str]:
    return stable_map_0.values()


@query
def stable_map_0_items() -> list[tuple[nat8, str]]:
    return stable_map_0.items()


@query
def stable_map_0_len() -> nat64:
    return stable_map_0.len()
