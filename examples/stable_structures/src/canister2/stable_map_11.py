from kybra import InsertError, opt, query, Record, StableBTreeMap, update, Variant
from kybra import nat, nat64


class BlogPost(Record):
    title: str


class User(Record):
    username: str
    blog_posts: list[BlogPost]


class StableMap11InsertResult(Variant, total=False):
    ok: opt[User]
    err: InsertError


stable_map_11 = StableBTreeMap[nat, User](
    memory_id=11, max_key_size=100, max_value_size=1_000)


@query
def stable_map_11_get(key: nat) -> opt[User]:
    return stable_map_11.get(key)


@update
def stable_map_11_insert(key: nat, value: User) -> StableMap11InsertResult:
    result = stable_map_11.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_11_remove(key: nat) -> opt[User]:
    return stable_map_11.remove(key)


@query
def stable_map_11_contains_key(key: nat) -> bool:
    return stable_map_11.contains_key(key)


@query
def stable_map_11_is_empty() -> bool:
    return stable_map_11.is_empty()


@query
def stable_map_11_keys() -> list[nat]:
    return stable_map_11.keys()


@query
def stable_map_11_values() -> list[User]:
    return stable_map_11.values()


@query
def stable_map_11_items() -> list[tuple[nat, User]]:
    return stable_map_11.items()


@query
def stable_map_11_len() -> nat64:
    return stable_map_11.len()
