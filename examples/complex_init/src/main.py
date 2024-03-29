from kybra import init, Opt, query, Record, Tuple, void


class User(Record):
    id: str


greeting: str = "Hello User"
user: Opt[User] = None


@init
def init_(tuple: Tuple[str, User]) -> void:
    global greeting
    global user

    greeting = tuple[0]
    user = tuple[1]


@query
def greet_user() -> str:
    return f"{greeting} {user['id'] if user else '??'}"
