from kybra import ic, query
from src.notifiers.types import NotifierFunc


@query
def get_notifier() -> NotifierFunc:
    return (ic.id(), "notify")
