# Stable Structures

## TLDR

-   96 GiB of stable memory
-   Persistent across upgrades
-   Familiar API
-   Must specify memory id
-   Must specify maximum key size
-   Must specify maximum value size
-   No migrations per memory id

Stable structures are data structures with familiar APIs that allow access to stable memory. Stable memory is a separate memory location from the heap that currently allows up to 96 GiB of storage. Stable memory persists automatically across upgrades.

Persistence on the Internet Computer (IC) is very important to understand. When a canister is upgraded (its code is changed after being initially deployed) its heap is wiped. This includes all global variables.

On the other hand, anything stored in stable memory will be preserved. Writing and reading to and from stable memory can be done with a [low-level API](./reference/stable_memory/stable_memory.md), but it is generally easier and preferable to use stable structures.

Kybra currently provides one stable structure called `StableBTreeMap`. It's similar to a Python dictionary or a map that you'd find in most languages, and has most of the common operations you'd expect such as reading, inserting, and removing values.

Here's how to define a simple `StableBTreeMap`. Each `StableBTreeMap` must be defined in the global scope (not within any functions or objects etc):

```python
from kybra import nat8, StableBTreeMap

map = StableBTreeMap[nat8, str](memory_id=3, max_key_size=100, max_value_size=1_000)
```

This is a `StableBTreeMap` with a key of type `nat8` and a value of type `str`. Key and value types can be any [Candid type](./candid.md).

This `StableBTreeMap` also has a `memory id` of `3`, a maximum key size of `100` bytes and a maximum value size of `1_000` bytes. You must statically specify the `memory id`, maximum key size, and maximum value sizes (they cannot be variables).

The `memory id` can be a number between `0` and `254`, but `memory ids` `0`, `1`, `2`, `252`, `253`, and `254` are currently reserved. We hope to reduce the complexity around `memory id` in the future, and perhaps remove the need to specify it entirely.

Each `StableBTreeMap` instance must have a unique `memory id`. Once a `memory id` is allocated, it cannot be used with a different `StableBTreeMap`. This means you can't create another `StableBTreeMap` using the same `memory id`, and you can't change the key or value types of an existing `StableBTreeMap`. [This problem will be addressed](https://github.com/demergent-labs/kybra/issues/215).

Here's an example showing all of the basic `StableBTreeMap` operations:

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


map = StableBTreeMap[Key, Value](memory_id=3, max_key_size=100, max_value_size=1_000)


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

With these basic operations you can build more complex CRUD database applications:

```python
import secrets

from kybra import (
    blob,
    ic,
    nat64,
    Opt,
    Principal,
    query,
    Record,
    StableBTreeMap,
    update,
    Variant,
    Vec,
)


class User(Record):
    id: Principal
    created_at: nat64
    recording_ids: Vec[Principal]
    username: str


class Recording(Record):
    id: Principal
    audio: blob
    created_at: nat64
    name: str
    user_id: Principal


users = StableBTreeMap[Principal, User](
    memory_id=3, max_key_size=38, max_value_size=100_000
)
recordings = StableBTreeMap[Principal, Recording](
    memory_id=4, max_key_size=38, max_value_size=5_000_000
)


@update
def create_user(username: str) -> User:
    id = generate_id()
    user: User = {
        "id": id,
        "created_at": ic.time(),
        "recording_ids": [],
        "username": username,
    }

    users.insert(user["id"], user)

    return user


@query
def read_users() -> Vec[User]:
    return users.values()


@query
def read_user_by_id(id: Principal) -> Opt[User]:
    return users.get(id)


class DeleteUserResult(Variant, total=False):
    Ok: User
    Err: "DeleteUserErr"


class DeleteUserErr(Variant, total=False):
    UserDoesNotExist: Principal


@update
def delete_user(id: Principal) -> DeleteUserResult:
    user = users.get(id)

    if user is None:
        return {"Err": {"UserDoesNotExist": id}}

    for recording_id in user["recording_ids"]:
        recordings.remove(recording_id)

    users.remove(user["id"])

    return {"Ok": user}


class CreateRecordingResult(Variant, total=False):
    Ok: Recording
    Err: "CreateRecordingErr"


class CreateRecordingErr(Variant, total=False):
    UserDoesNotExist: Principal


@update
def create_recording(
    audio: blob, name: str, user_id: Principal
) -> CreateRecordingResult:
    user = users.get(user_id)

    if user is None:
        return {"Err": {"UserDoesNotExist": user_id}}

    id = generate_id()
    recording: Recording = {
        "id": id,
        "audio": audio,
        "created_at": ic.time(),
        "name": name,
        "user_id": user_id,
    }

    recordings.insert(recording["id"], recording)

    updated_user: User = {
        "id": user["id"],
        "created_at": user["created_at"],
        "username": user["username"],
        "recording_ids": [*user["recording_ids"], recording["id"]],
    }

    users.insert(updated_user["id"], updated_user)

    return {"Ok": recording}


@query
def read_recordings() -> Vec[Recording]:
    return recordings.values()


@query
def read_recording_by_id(id: Principal) -> Opt[Recording]:
    return recordings.get(id)


class DeleteRecordingResult(Variant, total=False):
    Ok: Recording
    Err: "DeleteRecordingError"


class DeleteRecordingError(Variant, total=False):
    RecordingDoesNotExist: Principal
    UserDoesNotExist: Principal


@update
def delete_recording(id: Principal) -> DeleteRecordingResult:
    recording = recordings.get(id)

    if recording is None:
        return {"Err": {"RecordingDoesNotExist": id}}

    user = users.get(recording["user_id"])

    if user is None:
        return {"Err": {"UserDoesNotExist": recording["user_id"]}}

    updated_user: User = {
        "id": user["id"],
        "created_at": user["created_at"],
        "username": user["username"],
        "recording_ids": list(
            filter(
                lambda recording_id: recording_id.to_str() != recording["id"].to_str(),
                user["recording_ids"],
            )
        ),
    }

    users.insert(updated_user["id"], updated_user)

    recordings.remove(id)

    return {"Ok": recording}


def generate_id() -> Principal:
    random_bytes = secrets.token_bytes(29)

    return Principal.from_hex(random_bytes.hex())
```

The example above shows a very basic audio recording backend application. There are two types of entities that need to be stored, `User` and `Recording`. These are represented as `Candid` records.

Each entity gets its own `StableBTreeMap`:

```python
from kybra import blob, nat64, Principal, Record, StableBTreeMap, Vec


class User(Record):
    id: Principal
    created_at: nat64
    recording_ids: Vec[Principal]
    username: str


class Recording(Record):
    id: Principal
    audio: blob
    created_at: nat64
    name: str
    user_id: Principal


users = StableBTreeMap[Principal, User](
    memory_id=3, max_key_size=38, max_value_size=100_000
)
recordings = StableBTreeMap[Principal, Recording](
    memory_id=4, max_key_size=38, max_value_size=5_000_000
)
```

Notice that each `StableBTreeMap` has a unique `memory id`. The maximum key and value sizes are also set according to the expected application usage.

You can figure out the appropriate maximum key and value sizes by reasoning about your application and engaging in some trial and error using the `insert` method. Calling `insert` on a `StableBTreeMap` will throw an error which in some cases will have the information that you need to determine the maximum key or value size.

If you attempt to insert a key or value that is too large, the `KeyTooLarge` and `ValueTooLarge` errors will show you the size of the value that you attempted to insert. You can increase the maximum key or value size based on the information you receive from the `KeyTooLarge` and `ValueTooLarge` errors and try inserting again.

Thus through some trial and error you can whittle your way to a correct solution. In some cases all of your values will have an obvious static maximum size. In the audio recording example, trial and error revealed that `Principal` is most likely always 38 bytes, thus the maximum key size is set to 38.

Maximum value sizes can be more tricky to figure out, especially if the values are records or variants with dynamic fields such as arrays. `User` has one such dynamic field, `recording_ids`. Since each recording id is a `Principal`, we know that each will take up 38 bytes. The other fields on `User` shouldn't take up too many bytes so we'll ignore them for our analysis.

We've set the maximum value size of `User` to be 100_000 bytes. If we divide 100_00 by 38, we get ~2_631. This will result in each user being able to store around that many recordings. That's acceptable for our example, and so we'll go with it.

As for `Recording`, the largest dynamic field is `audio`, which will be the actual bytes of the audio recording. We've set the maximum value size here to 5_000_000, which should allow for recordings of ~5 MB in size. That seems reasonable for our example, and so we'll go with it.

As you can see, finding the correct maximum key and value sizes is a bit of an art right now. Combining some trial and error with reasoning about your specific application should get you a working solution in most cases. It's our hope that the need to specify maximum key and value sizes will be removed in the future.

## Caveats

### memory ids

The `memory id` can be a number between `0` and `254`, but `memory ids` `0`, `1`, `2`, and `254` are currently reserved. We hope to reduce the complexity around `memory id` in the future, and perhaps remove the need for it entirely.

### Keys

You should be wary when using `float64`, `float32`, `service`, or `func` in any type that is a key for a stable structure. These types do not have the ability to be strictly ordered in all cases. `service` and `func` will have no order. `float64` and `float32` will treat `NaN` as less than any other type. These caveats may impact key performance.
