from kybra import blob, opt, query, Record, StableBTreeMap, update, Variant
from kybra import nat, nat64, nat32, nat16, nat8
from kybra import float32, float64


class User(Record):
    username: str
    blog_posts: list['BlogPost']


class BlogPost(Record):
    title: str


class Reaction(Variant):
    Happy: None
    Sad: None


stable_map_0 = StableBTreeMap[nat8, str](
    memory_id=0, max_key_size=100, max_value_size=100)
stable_map_1 = StableBTreeMap[nat16, blob](
    memory_id=1, max_key_size=100, max_value_size=1_000)
stable_map_2 = StableBTreeMap[nat32, nat](
    memory_id=2, max_key_size=100, max_value_size=1_000)
stable_map_3 = StableBTreeMap[Reaction, int](
    memory_id=3, max_key_size=100, max_value_size=1_000)
stable_map_4 = StableBTreeMap[User, float32](
    memory_id=4, max_key_size=100, max_value_size=1_000)
stable_map_5 = StableBTreeMap[opt[str], float64](
    memory_id=5, max_key_size=100, max_value_size=1_000)
stable_map_6 = StableBTreeMap[list[nat64], bool](
    memory_id=6, max_key_size=100, max_value_size=1_000)


@query
def stable_map_0_get(key: nat8) -> opt[str]:
    return stable_map_0.get(key)


@update
def stable_map_0_insert(key: nat8, value: str):
    return stable_map_0.insert(key, value)


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


@query
def stable_map_1_get(key: nat16) -> opt[blob]:
    return stable_map_1.get(key)


@update
def stable_map_1_insert(key: nat16, value: blob):
    return stable_map_1.insert(key, value)


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


@query
def stable_map_2_get(key: nat32) -> opt[nat]:
    return stable_map_2.get(key)


@update
def stable_map_2_insert(key: nat32, value: nat):
    return stable_map_2.insert(key, value)


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


@query
def stable_map_3_get(key: Reaction) -> opt[int]:
    return stable_map_3.get(key)


@update
def stable_map_3_insert(key: Reaction, value: int):
    return stable_map_3.insert(key, value)


@update
def stable_map_3_remove(key: Reaction) -> opt[int]:
    return stable_map_3.remove(key)


@query
def stable_map_3_contains_key(key: Reaction) -> bool:
    return stable_map_3.contains_key(key)


@query
def stable_map_3_is_empty() -> bool:
    return stable_map_3.is_empty()


@query
def stable_map_3_keys() -> list[Reaction]:
    return stable_map_3.keys()


@query
def stable_map_3_values() -> list[int]:
    return stable_map_3.values()


@query
def stable_map_3_items() -> list[tuple[Reaction, int]]:
    return stable_map_3.items()


@query
def stable_map_3_len() -> nat64:
    return stable_map_3.len()


@query
def stable_map_4_get(key: User) -> opt[float32]:
    return stable_map_4.get(key)


@update
def stable_map_4_insert(key: User, value: float32):
    return stable_map_4.insert(key, value)


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


@query
def stable_map_5_get(key: opt[str]) -> opt[float64]:
    return stable_map_5.get(key)


@update
def stable_map_5_insert(key: opt[str], value: float64):
    return stable_map_5.insert(key, value)


@update
def stable_map_5_remove(key: opt[str]) -> opt[float64]:
    return stable_map_5.remove(key)


@query
def stable_map_5_contains_key(key: opt[str]) -> bool:
    return stable_map_5.contains_key(key)


@query
def stable_map_5_is_empty() -> bool:
    return stable_map_5.is_empty()


@query
def stable_map_5_keys() -> list[opt[str]]:
    return stable_map_5.keys()


@query
def stable_map_5_values() -> list[float64]:
    return stable_map_5.values()


@query
def stable_map_5_items() -> list[tuple[opt[str], float64]]:
    return stable_map_5.items()


@query
def stable_map_5_len() -> nat64:
    return stable_map_5.len()


@query
def stable_map_6_get(key: list[nat64]) -> opt[bool]:
    return stable_map_6.get(key)


@update
def stable_map_6_insert(key: list[nat64], value: bool):
    return stable_map_6.insert(key, value)


@update
def stable_map_6_remove(key: list[nat64]) -> opt[bool]:
    return stable_map_6.remove(key)


@query
def stable_map_6_contains_key(key: list[nat64]) -> bool:
    return stable_map_6.contains_key(key)


@query
def stable_map_6_is_empty() -> bool:
    return stable_map_6.is_empty()


@query
def stable_map_6_keys() -> list[list[nat64]]:
    return stable_map_6.keys()


@query
def stable_map_6_values() -> list[bool]:
    return stable_map_6.values()


@query
def stable_map_6_items() -> list[tuple[list[nat64], bool]]:
    return stable_map_6.items()


@query
def stable_map_6_len() -> nat64:
    return stable_map_6.len()
