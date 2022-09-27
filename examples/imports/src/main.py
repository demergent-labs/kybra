from kybra import query
import import1
import import2
import import2.import3

@query
def get_import1_message() -> str:
    return import1.get_message()

@query
def get_import2_message() -> str:
    return import2.get_message()

@query
def get_import3_message() -> str:
    return import2.import3.get_message()
