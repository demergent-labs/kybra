from kybra import InsertError, blob, opt, query, StableBTreeMap, update, Variant
from kybra import nat64, nat16


class StableMap1InsertResult(Variant, total=False):
    ok: opt[blob]
    err: InsertError


stable_map_1 = StableBTreeMap[nat16, blob](
    memory_id=1, max_key_size=100, max_value_size=1_000)


@query
def stable_map_1_get(key: nat16) -> opt[blob]:
    return stable_map_1.get(key)


@update
def stable_map_1_insert(key: nat16, value: blob) -> StableMap1InsertResult:
    result = stable_map_1.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_1_remove(key: nat16) -> opt[blob]:
    return stable_map_1.remove(key)


@query
def stable_map_1_contains_key(key: nat16) -> bool:
    return stable_map_1.contains_key(key)


@query
def stable_map_1_is_empty() -> bool:
    return stable_map_1.is_empty()


@query
def stable_map_1_keys() -> list[nat16]:
    return stable_map_1.keys()


@query
def stable_map_1_values() -> list[blob]:
    return stable_map_1.values()


@query
def stable_map_1_items() -> list[tuple[nat16, blob]]:
    return stable_map_1.items()


@query
def stable_map_1_len() -> nat64:
    return stable_map_1.len()
