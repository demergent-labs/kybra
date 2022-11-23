from kybra import Principal, query
from src.notifiers.types import NotifierFunc


@query
def get_notifier() -> NotifierFunc:
    return (Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"), "notify")
