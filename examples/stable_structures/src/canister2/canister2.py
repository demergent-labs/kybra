from kybra import InsertError, blob, null, opt, Principal, query, Record, StableBTreeMap, update, Variant
from kybra import nat, nat64
from kybra import float32, float64


class BlogPost(Record):
    title: str


class User(Record):
    username: str
    blog_posts: list[BlogPost]


class Reaction(Variant):
    Happy: None
    Sad: None


stable_map_7 = StableBTreeMap[null, null](
    memory_id=7, max_key_size=100, max_value_size=1_000)
stable_map_8 = StableBTreeMap[bool, None](
    memory_id=8, max_key_size=100, max_value_size=1_000)
stable_map_9 = StableBTreeMap[float64, list[str]](
    memory_id=9, max_key_size=100, max_value_size=1_000)
stable_map_10 = StableBTreeMap[float32, opt[bool]](
    memory_id=10, max_key_size=100, max_value_size=1_000)
stable_map_11 = StableBTreeMap[nat, User](
    memory_id=11, max_key_size=100, max_value_size=1_000)
stable_map_12 = StableBTreeMap[blob, Reaction](
    memory_id=12, max_key_size=100, max_value_size=1_000)
stable_map_13 = StableBTreeMap[str, Principal](
    memory_id=13, max_key_size=100, max_value_size=1_000)


class StableMap7InsertResult(Variant, total=False):
    ok: opt[null]
    err: InsertError


class StableMap8InsertResult(Variant, total=False):
    ok: opt[None]
    err: InsertError


class StableMap9InsertResult(Variant, total=False):
    ok: opt[list[str]]
    err: InsertError


class StableMap10InsertResult(Variant, total=False):
    ok: opt[opt[bool]]
    err: InsertError


class StableMap11InsertResult(Variant, total=False):
    ok: opt[User]
    err: InsertError


class StableMap12InsertResult(Variant, total=False):
    ok: opt[Reaction]
    err: InsertError


class StableMap13InsertResult(Variant, total=False):
    ok: opt[Principal]
    err: InsertError


@query
def stable_map_7_get(key: null) -> opt[null]:
    return stable_map_7.get(key)


@update
def stable_map_7_insert(key: null, value: null) -> StableMap7InsertResult:
    result = stable_map_7.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_7_remove(key: null) -> opt[null]:
    return stable_map_7.remove(key)


@query
def stable_map_7_contains_key(key: null) -> bool:
    return stable_map_7.contains_key(key)


@query
def stable_map_7_is_empty() -> bool:
    return stable_map_7.is_empty()


@query
def stable_map_7_keys() -> list[null]:
    return stable_map_7.keys()


@query
def stable_map_7_values() -> list[null]:
    return stable_map_7.values()


@query
def stable_map_7_items() -> list[tuple[null, null]]:
    return stable_map_7.items()


@query
def stable_map_7_len() -> nat64:
    return stable_map_7.len()


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


@query
def stable_map_9_get(key: float64) -> opt[list[str]]:
    return stable_map_9.get(key)


@update
def stable_map_9_insert(key: float64, value: list[str]) -> StableMap9InsertResult:
    result = stable_map_9.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_9_remove(key: float64) -> opt[list[str]]:
    return stable_map_9.remove(key)


@query
def stable_map_9_contains_key(key: float64) -> bool:
    return stable_map_9.contains_key(key)


@query
def stable_map_9_is_empty() -> bool:
    return stable_map_9.is_empty()


@query
def stable_map_9_keys() -> list[float64]:
    return stable_map_9.keys()


@query
def stable_map_9_values() -> list[list[str]]:
    return stable_map_9.values()


@query
def stable_map_9_items() -> list[tuple[float64, list[str]]]:
    return stable_map_9.items()


@query
def stable_map_9_len() -> nat64:
    return stable_map_9.len()


@query
def stable_map_10_get(key: float32) -> opt[opt[bool]]:
    return stable_map_10.get(key)


@update
def stable_map_10_insert(key: float32, value: opt[bool]) -> StableMap10InsertResult:
    result = stable_map_10.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_10_remove(key: float32) -> opt[opt[bool]]:
    return stable_map_10.remove(key)


@query
def stable_map_10_contains_key(key: float32) -> bool:
    return stable_map_10.contains_key(key)


@query
def stable_map_10_is_empty() -> bool:
    return stable_map_10.is_empty()


@query
def stable_map_10_keys() -> list[float32]:
    return stable_map_10.keys()


@query
def stable_map_10_values() -> list[opt[bool]]:
    return stable_map_10.values()


@query
def stable_map_10_items() -> list[tuple[float32, opt[bool]]]:
    return stable_map_10.items()


@query
def stable_map_10_len() -> nat64:
    return stable_map_10.len()


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


@query
def stable_map_12_get(key: blob) -> opt[Reaction]:
    return stable_map_12.get(key)


@update
def stable_map_12_insert(key: blob, value: Reaction) -> StableMap12InsertResult:
    result = stable_map_12.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_12_remove(key: blob) -> opt[Reaction]:
    return stable_map_12.remove(key)


@query
def stable_map_12_contains_key(key: blob) -> bool:
    return stable_map_12.contains_key(key)


@query
def stable_map_12_is_empty() -> bool:
    return stable_map_12.is_empty()


@query
def stable_map_12_keys() -> list[blob]:
    return stable_map_12.keys()


@query
def stable_map_12_values() -> list[Reaction]:
    return stable_map_12.values()


@query
def stable_map_12_items() -> list[tuple[blob, Reaction]]:
    return stable_map_12.items()


@query
def stable_map_12_len() -> nat64:
    return stable_map_12.len()


@query
def stable_map_13_get(key: str) -> opt[Principal]:
    return stable_map_13.get(key)


@update
def stable_map_13_insert(key: str, value: Principal) -> StableMap13InsertResult:
    result = stable_map_13.insert(key, value)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }


@update
def stable_map_13_remove(key: str) -> opt[Principal]:
    return stable_map_13.remove(key)


@query
def stable_map_13_contains_key(key: str) -> bool:
    return stable_map_13.contains_key(key)


@query
def stable_map_13_is_empty() -> bool:
    return stable_map_13.is_empty()


@query
def stable_map_13_keys() -> list[str]:
    return stable_map_13.keys()


@query
def stable_map_13_values() -> list[Principal]:
    return stable_map_13.values()


@query
def stable_map_13_items() -> list[tuple[str, Principal]]:
    return stable_map_13.items()


@query
def stable_map_13_len() -> nat64:
    return stable_map_13.len()
