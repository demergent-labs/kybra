from kybra import opt, query, update, void

Store = dict[str, str]

store: Store = {}


@query
def get(key: str) -> opt[str]:
    return store.get(key, None)


@update
def set(key: str, value: str) -> void:
    store[key] = value
