# stable structures

This section is a work in progress.

Examples:

-   [audio_recorder](https://github.com/demergent-labs/kybra/tree/main/examples/audio_recorder)
-   [ethereum_json_rpc](https://github.com/demergent-labs/kybra/tree/main/examples/ethereum_json_rpc)
-   [func_types](https://github.com/demergent-labs/kybra/tree/main/examples/func_types)
-   [http_counter](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/http_counter)
-   [persistent-storage](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/persistent-storage)
-   [pre_and_post_upgrade](https://github.com/demergent-labs/kybra/tree/main/examples/pre_and_post_upgrade)
-   [stable_structures](https://github.com/demergent-labs/kybra/tree/main/examples/stable_structures)

```python
from kybra import (
    Alias,
    nat64,
    nat8,
    Opt,
    query,
    StableBTreeMap,
    Tuple,
    update,
    Vec,
)

Key = Alias[nat8]
Value = Alias[str]


map = StableBTreeMap[Key, Value](memory_id=0, max_key_size=100, max_value_size=1_000)


@query
def contains_key(key: Key) -> bool:
    return map.contains_key(key)


@query
def get(key: Key) -> Opt[Value]:
    return map.get(key)


@update
def insert(key: Key, value: Value) -> Opt[Value]:
    return map.insert(key, value)


@query
def is_empty() -> bool:
    return map.is_empty()


@query
def items() -> Vec[Tuple[Key, Value]]:
    return map.items()


@query
def keys() -> Vec[Key]:
    return map.keys()


@query
def len() -> nat64:
    return map.len()


@update
def remove(key: Key) -> Opt[Value]:
    return map.remove(key)


@query
def values() -> Vec[Value]:
    return map.values()
```
