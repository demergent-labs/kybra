from kybra import init, opt, query, Record


class User(Record):
    id: str


greeting: str = "Hello User"
user: opt[User] = None


@init
def init_(tuple: tuple[str, User]):
    global greeting
    global user

    greeting = tuple[0]
    user = tuple[1]


@query
def greet_user() -> str:
    if not user:
        name = "??"
    else:
        name = user["id"]

    return f"{greeting} {name}"
