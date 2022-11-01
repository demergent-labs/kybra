from kybra import init, nat64, post_upgrade, pre_upgrade, query, Record, update

class Entry(Record):
    key: str
    value: nat64

entries: dict[str, nat64] = {}

initialized: bool = False

@init
def init_():
    print("init")
    # TODO figure out how to do stable storage

@pre_upgrade
def pre_upgrade_():
    print("preUpgrade")

    #TODO more stuff with stable storage

@post_upgrade
def post_upgrade_():
    print("postUpgrade")

    #TODO more stuff with stable storage

@update
def setEntry(entry: Entry):
    global entries

    entries[entry['key']] = entry['value']

@query
def getEntries() -> list[Entry]:
    return [{'key': key, 'value': entries[key]} for key in entries.keys()]
