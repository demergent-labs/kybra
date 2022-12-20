from kybra import nat64, opt, query, StableBTreeMap, update

stable_map_0 = StableBTreeMap[nat64, str](memory_id=0, max_key_size=100, max_value_size=100)
# stable_map_1 = StableBTreeMap[nat64, nat8](memory_id=1, max_key_size=10, max_value_size=100)

@query
def get(key: nat64) -> opt[str]:
    return stable_map_0.get(key)

@update
def set(key: nat64, value: str):
    return stable_map_0.insert(key, value)
