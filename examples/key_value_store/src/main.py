from kybra import Opt, query, update, void

Store = dict[str, str]

store: Store = {}


@query
def get(key: str) -> Opt[str]:
    return store.get(key, None)


@update
def set(key: str, value: str) -> void:
    store[key] = value
