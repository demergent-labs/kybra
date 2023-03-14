from kybra import opt, Record, query, update, void


class Entry(Record):
    desc: str
    phone: str


phone_book: dict[str, Entry] = {}


@update
def insert(name: str, entry: Entry) -> void:
    phone_book[name] = entry


@query
def lookup(name: str) -> opt[Entry]:
    return phone_book[name] if name in phone_book else None
