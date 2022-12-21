from kybra import blob, null, opt, Principal, query, Record, StableBTreeMap, update, Variant
from kybra import nat, nat64, nat32, nat16, nat8
# from kybra import int64, int32, int16, int8
from kybra import float32, float64

# TODO text
# TODO blob
# TODO nat
# TODO int
# TODO intN
# TODO nat
# TODO natN
# TODO float32 and float64
# TODO bool
# TODO null
# TODO vec
# TODO opt
# TODO record
# TODO variant
# TODO func
# TODO principal
# TODO reserved
# TODO empty

class User(Record):
    username: str
    blog_posts: list['BlogPost']

class BlogPost(Record):
    title: str

class Reaction(Variant):
    Happy: None
    Sad: None

stable_map_0 = StableBTreeMap[nat8, str](memory_id=0, max_key_size=100, max_value_size=100)
stable_map_1 = StableBTreeMap[nat16, blob](memory_id=1, max_key_size=100, max_value_size=1_000)
stable_map_2 = StableBTreeMap[nat32, nat](memory_id=2, max_key_size=100, max_value_size=1_000)
stable_map_3 = StableBTreeMap[Reaction, int](memory_id=3, max_key_size=100, max_value_size=1_000)

# TODO we can add all of the int and nat varieties if we want

stable_map_4 = StableBTreeMap[User, float32](memory_id=4, max_key_size=100, max_value_size=1_000)
stable_map_5 = StableBTreeMap[opt[str], float64](memory_id=5, max_key_size=100, max_value_size=1_000)
stable_map_6 = StableBTreeMap[list[nat64], bool](memory_id=6, max_key_size=100, max_value_size=1_000)
stable_map_7 = StableBTreeMap[null, null](memory_id=7, max_key_size=100, max_value_size=1_000)
stable_map_8 = StableBTreeMap[bool, None](memory_id=8, max_key_size=100, max_value_size=1_000)

# TODO should test lists of many things, including lists of lists
stable_map_9 = StableBTreeMap[float64, list[str]](memory_id=9, max_key_size=100, max_value_size=1_000)
stable_map_10 = StableBTreeMap[float32, opt[bool]](memory_id=10, max_key_size=100, max_value_size=1_000)
stable_map_11 = StableBTreeMap[nat, User](memory_id=11, max_key_size=100, max_value_size=1_000)
stable_map_12 = StableBTreeMap[blob, Reaction](memory_id=12, max_key_size=100, max_value_size=1_000)
stable_map_13 = StableBTreeMap[str, Principal](memory_id=13, max_key_size=100, max_value_size=1_000)

@query
def get_stable_map_0(key: nat8) -> opt[str]:
    return stable_map_0.get(key)

@update
def set_stable_map_0(key: nat8, value: str):
    return stable_map_0.insert(key, value)

@update
def remove_stable_map_0(key: nat8) -> opt[str]:
    return stable_map_0.remove(key)

@query
def contains_key_stable_map_0(key: nat8) -> bool:
    return stable_map_0.contains_key(key)

@query
def is_empty_stable_map_0() -> bool:
    return stable_map_0.is_empty()

@query
def len_stable_map_0() -> nat64:
    return stable_map_0.len()

@query
def get_stable_map_1(key: nat16) -> opt[blob]:
    return stable_map_1.get(key)

@update
def set_stable_map_1(key: nat16, value: blob):
    return stable_map_1.insert(key, value)

@query
def get_stable_map_2(key: nat32) -> opt[nat]:
    return stable_map_2.get(key)

@update
def set_stable_map_2(key: nat32, value: nat):
    return stable_map_2.insert(key, value)

@query
def get_stable_map_3(key: Reaction) -> opt[int]:
    return stable_map_3.get(key)

@update
def set_stable_map_3(key: Reaction, value: int):
    return stable_map_3.insert(key, value)

@query
def get_stable_map_4(key: User) -> opt[float32]:
    return stable_map_4.get(key)

@update
def set_stable_map_4(key: User, value: float32):
    return stable_map_4.insert(key, value)

@query
def get_stable_map_5(key: opt[str]) -> opt[float64]:
    return stable_map_5.get(key)

@update
def set_stable_map_5(key: opt[str], value: float64):
    return stable_map_5.insert(key, value)

@query
def get_stable_map_6(key: list[nat64]) -> opt[bool]:
    return stable_map_6.get(key)

@update
def set_stable_map_6(key: list[nat64], value: bool):
    return stable_map_6.insert(key, value)

@query
def get_stable_map_7(key: null) -> opt[null]:
    return stable_map_7.get(key)

@update
def set_stable_map_7(key: null, value: null):
    return stable_map_7.insert(key, value)

@query
def get_stable_map_8(key: bool) -> opt[None]:
    return stable_map_8.get(key)

@update
def set_stable_map_8(key: bool, value: None):
    return stable_map_8.insert(key, value)

@query
def get_stable_map_9(key: float64) -> opt[list[str]]:
    return stable_map_9.get(key)

@update
def set_stable_map_9(key: float64, value: list[str]):
    return stable_map_9.insert(key, value)

@query
def get_stable_map_10(key: float32) -> opt[opt[bool]]:
    return stable_map_10.get(key)

@update
def set_stable_map_10(key: float32, value: opt[bool]):
    return stable_map_10.insert(key, value)

@query
def get_stable_map_11(key: nat) -> opt[User]:
    return stable_map_11.get(key)

@update
def set_stable_map_11(key: nat, value: User):
    return stable_map_11.insert(key, value)

@query
def get_stable_map_12(key: blob) -> opt[Reaction]:
    return stable_map_12.get(key)

@update
def set_stable_map_12(key: blob, value: Reaction):
    return stable_map_12.insert(key, value)

@query
def get_stable_map_13(key: str) -> opt[Principal]:
    return stable_map_13.get(key)

@update
def set_stable_map_13(key: str, value: Principal):
    return stable_map_13.insert(key, value)
