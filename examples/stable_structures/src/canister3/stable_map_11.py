from kybra import (
    InsertError,
    match,
    opt,
    query,
    Record,
    StableBTreeMap,
    update,
    Variant,
)
from kybra import nat, nat64


class BlogPost(Record):
    title: str


class User(Record):
    username: str
    posts: list[BlogPost]


class StableMap11InsertResult(Variant, total=False):
    Ok: opt[User]
    Err: InsertError


stable_map11 = StableBTreeMap[nat, User](
    memory_id=11, max_key_size=100, max_value_size=1_000
)


@query
def stable_map11_get(key: nat) -> opt[User]:
    return stable_map11.get(key)


@update
def stable_map11_insert(key: nat, value: User) -> StableMap11InsertResult:
    result = stable_map11.insert(key, value)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})


@update
def stable_map11_remove(key: nat) -> opt[User]:
    return stable_map11.remove(key)


@query
def stable_map11_contains_key(key: nat) -> bool:
    return stable_map11.contains_key(key)


@query
def stable_map11_is_empty() -> bool:
    return stable_map11.is_empty()


@query
def stable_map11_keys() -> list[nat]:
    return stable_map11.keys()


@query
def stable_map11_values() -> list[User]:
    return stable_map11.values()


@query
def stable_map11_items() -> list[tuple[nat, User]]:
    return stable_map11.items()


@query
def stable_map11_len() -> nat64:
    return stable_map11.len()
