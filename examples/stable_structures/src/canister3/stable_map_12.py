from kybra import (
    InsertError,
    blob,
    match,
    null,
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
    Variant,
)
from kybra import nat64


class Reaction(Variant):
    Happy: null
    Sad: null


class StableMap12InsertResult(Variant, total=False):
    Ok: Opt[Reaction]
    Err: InsertError


stable_map12 = StableBTreeMap[blob, Reaction](
    memory_id=12, max_key_size=100, max_value_size=1_000
)


@query
def stable_map12_get(key: blob) -> Opt[Reaction]:
    return stable_map12.get(key)


@update
def stable_map12_insert(key: blob, value: Reaction) -> StableMap12InsertResult:
    result = stable_map12.insert(key, value)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


@update
def stable_map12_remove(key: blob) -> Opt[Reaction]:
    return stable_map12.remove(key)


@query
def stable_map12_contains_key(key: blob) -> bool:
    return stable_map12.contains_key(key)


@query
def stable_map12_is_empty() -> bool:
    return stable_map12.is_empty()


@query
def stable_map12_keys() -> Vec[blob]:
    return stable_map12.keys()


@query
def stable_map12_values() -> Vec[Reaction]:
    return stable_map12.values()


@query
def stable_map12_items() -> Vec[Tuple[blob, Reaction]]:
    return stable_map12.items()


@query
def stable_map12_len() -> nat64:
    return stable_map12.len()
