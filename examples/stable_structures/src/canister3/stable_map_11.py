from kybra import (
    Opt,
    query,
    Record,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
)
from kybra import nat, nat64


class BlogPost(Record):
    title: str


class User(Record):
    username: str
    posts: Vec[BlogPost]


stable_map11 = StableBTreeMap[nat, User](
    memory_id=11, max_key_size=100, max_value_size=1_000
)


@query
def stable_map11_get(key: nat) -> Opt[User]:
    return stable_map11.get(key)


@update
def stable_map11_insert(key: nat, value: User) -> Opt[User]:
    return stable_map11.insert(key, value)


@update
def stable_map11_remove(key: nat) -> Opt[User]:
    return stable_map11.remove(key)


@query
def stable_map11_contains_key(key: nat) -> bool:
    return stable_map11.contains_key(key)


@query
def stable_map11_is_empty() -> bool:
    return stable_map11.is_empty()


@query
def stable_map11_keys() -> Vec[nat]:
    return stable_map11.keys()


@query
def stable_map11_values() -> Vec[User]:
    return stable_map11.values()


@query
def stable_map11_items() -> Vec[Tuple[nat, User]]:
    return stable_map11.items()


@query
def stable_map11_len() -> nat64:
    return stable_map11.len()
