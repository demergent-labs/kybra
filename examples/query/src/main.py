from kybra import query, update
from library import hello_there
from other_stuff.monkey import print_monkey

@query
def concat(x: str, y: str) -> str:
    return x + y

@query
def add(x: int, y: int) -> int:
    return x + y

saved_message = ''

@update
def record_message(message: str) -> str:
    global saved_message
    saved_message = message
    return saved_message

@query
def get_message() -> str:
    return saved_message

@query
def boolean_logic(x: bool, y: bool) -> bool:
    return x and y

@query
def library_hello_there() -> str:
    return hello_there()

@query
def print_the_monkey() -> str:
    return print_monkey()
