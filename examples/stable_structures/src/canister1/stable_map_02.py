from kybra import InsertError, opt, query, StableBTreeMap, update, Variant
from kybra import nat, nat64, nat32


class StableMap2InsertResult(Variant, total=False):
    ok: opt[nat]
    err: InsertError


stable_map_2 = StableBTreeMap[nat32, nat](
    memory_id=2, max_key_size=100, max_value_size=1_000)


@query
def stable_map_2_get(key: nat32) -> opt[nat]:
    return stable_map_2.get(key)


@update
def stable_map_2_insert(key: nat32, value: nat) -> StableMap2InsertResult:
    result = stable_map_2.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_2_remove(key: nat32) -> opt[nat]:
    return stable_map_2.remove(key)


@query
def stable_map_2_contains_key(key: nat32) -> bool:
    return stable_map_2.contains_key(key)


@query
def stable_map_2_is_empty() -> bool:
    return stable_map_2.is_empty()


@query
def stable_map_2_keys() -> list[nat32]:
    return stable_map_2.keys()


@query
def stable_map_2_values() -> list[nat]:
    return stable_map_2.values()


@query
def stable_map_2_items() -> list[tuple[nat32, nat]]:
    return stable_map_2.items()


@query
def stable_map_2_len() -> nat64:
    return stable_map_2.len()
