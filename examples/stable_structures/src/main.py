from kybra import nat64, opt, query, Record, StableBTreeMap, update

class User(Record):
    username: str

stable_map_0 = StableBTreeMap[nat64, str](memory_id=0, max_key_size=100, max_value_size=100)
stable_map_1 = StableBTreeMap[nat64, bool](memory_id=1, max_key_size=100, max_value_size=100)
stable_map_2 = StableBTreeMap[nat64, User](memory_id=2, max_key_size=100, max_value_size=1000)

@query
def get(key: nat64) -> opt[str]:
    return stable_map_0.get(key)

@update
def set(key: nat64, value: str):
    return stable_map_0.insert(key, value)

@query
def get_bool(key: nat64) -> opt[bool]:
    return stable_map_1.get(key)

@update
def set_bool(key: nat64, value: bool):
    return stable_map_1.insert(key, value)

@query
def get_user(key: nat64) -> opt[User]:
    return stable_map_2.get(key)

@update
def set_user(key: nat64, value: User):
    return stable_map_2.insert(key, value)
