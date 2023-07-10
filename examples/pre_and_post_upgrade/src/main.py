from kybra import (
    ic,
    init,
    nat64,
    post_upgrade,
    pre_upgrade,
    query,
    Record,
    StableBTreeMap,
    update,
    Vec,
    void,
)
from typing import TypedDict


class StableStorage(TypedDict):
    entries: Vec["Entry"]


class Entry(Record):
    key: str
    value: nat64


stable_storage = StableBTreeMap[str, Vec[Entry]](
    memory_id=3, max_key_size=100, max_value_size=100
)

entries: dict[str, nat64] = {}


@init
def init_() -> void:
    ic.print("init_")

    stable_storage.insert("entries", [])


@pre_upgrade
def pre_upgrade_() -> void:
    ic.print("pre_upgrade_")

    stable_storage.insert(
        "entries",
        list(map(lambda item: {"key": item[0], "value": item[1]}, entries.items())),
    )


@post_upgrade
def post_upgrade_() -> void:
    ic.print("post_upgrade_")

    stable_entries = stable_storage.get("entries")

    if stable_entries is not None:
        # TODO I would prefer to use a reduce, but it has been moved to functools
        # TODO and we need the Wasm binary limit of the IC lifted to access that stdlib
        for stable_entry in stable_entries:
            entries[stable_entry["key"]] = stable_entry["value"]


@update
def set_entry(entry: Entry) -> void:
    entries[entry["key"]] = entry["value"]


@query
def get_entries() -> Vec[Entry]:
    return [{"key": key, "value": entries[key]} for key in entries.keys()]
