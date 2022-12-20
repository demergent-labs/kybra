from kybra import nat8, nat64, opt, query, StableBTreeMap, update

stable_map_0 = StableBTreeMap[nat64, nat64](memory_id=0, max_key_size=10, max_value_size=100)
stable_map_1 = StableBTreeMap[nat64, nat8](memory_id=1, max_key_size=10, max_value_size=100)

@query
def get(key: nat64) -> opt[nat64]:
    return stable_map_0.get(key)

@update
def set(key: nat64, value: nat64):
    return stable_map_0.insert(key, value)
