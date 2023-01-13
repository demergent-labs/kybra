from kybra import InsertError, opt, query, StableBTreeMap, update, Variant
from kybra import nat64

stable_map_8 = StableBTreeMap[bool, None](
    memory_id=8, max_key_size=100, max_value_size=1_000)


class StableMap8InsertResult(Variant, total=False):
    ok: opt[None]
    err: InsertError


@query
def stable_map_8_get(key: bool) -> opt[None]:
    return stable_map_8.get(key)


@update
def stable_map_8_insert(key: bool, value: None) -> StableMap8InsertResult:
    result = stable_map_8.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_8_remove(key: bool) -> opt[None]:
    return stable_map_8.remove(key)


@query
def stable_map_8_contains_key(key: bool) -> bool:
    return stable_map_8.contains_key(key)


@query
def stable_map_8_is_empty() -> bool:
    return stable_map_8.is_empty()


@query
def stable_map_8_keys() -> list[bool]:
    return stable_map_8.keys()


@query
def stable_map_8_values() -> list[None]:
    return stable_map_8.values()


@query
def stable_map_8_items() -> list[tuple[bool, None]]:
    return stable_map_8.items()


@query
def stable_map_8_len() -> nat64:
    return stable_map_8.len()
