from kybra import InsertError, opt, query, StableBTreeMap, update, Variant, Record
from kybra import nat64
from kybra import float32


class BlogPost(Record):
    title: str


class User(Record):
    username: str
    blog_posts: list['BlogPost']


class StableMap4InsertResult(Variant, total=False):
    Ok: opt[float32]
    Err: InsertError


stable_map_4 = StableBTreeMap[User, float32](
    memory_id=4, max_key_size=100, max_value_size=1_000)


@query
def stable_map_4_get(key: User) -> opt[float32]:
    return stable_map_4.get(key)


@update
def stable_map_4_insert(key: User, value: float32) -> StableMap4InsertResult:
    result = stable_map_4.insert(key, value)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def stable_map_4_remove(key: User) -> opt[float32]:
    return stable_map_4.remove(key)


@query
def stable_map_4_contains_key(key: User) -> bool:
    return stable_map_4.contains_key(key)


@query
def stable_map_4_is_empty() -> bool:
    return stable_map_4.is_empty()


@query
def stable_map_4_keys() -> list[User]:
    return stable_map_4.keys()


@query
def stable_map_4_values() -> list[float32]:
    return stable_map_4.values()


@query
def stable_map_4_items() -> list[tuple[User, float32]]:
    return stable_map_4.items()


@query
def stable_map_4_len() -> nat64:
    return stable_map_4.len()
