from kybra import ic, init, nat64, post_upgrade, pre_upgrade, query, Record, update
from typing import TypedDict

class StableStorage(TypedDict):
    entries: list['Entry']

class Entry(Record):
    key: str
    value: nat64

stable_storage: StableStorage = ic.stable_storage()

entries: dict[str, nat64] = {}

@init
def init_():
    ic.print("init_")

    stable_storage['entries'] = []

@pre_upgrade
def pre_upgrade_():
    ic.print("pre_upgrade_")

    stable_storage['entries'] = list(
        map(lambda item: {
            'key': item[0],
            'value': item[1]
        }, entries.items())
    )

@post_upgrade
def post_upgrade_():
    ic.print("post_upgrade_")

    # TODO I would prefer to use a reduce, but it has been moved to functools
    # TODO and we need the Wasm binary limit of the IC lifted to access that stdlib
    for stable_entry in stable_storage['entries']:
        entries[stable_entry['key']] = stable_entry['value']

@update
def set_entry(entry: Entry):
    entries[entry['key']] = entry['value']

@query
def get_entries() -> list[Entry]:
    return [{'key': key, 'value': entries[key]} for key in entries.keys()]
