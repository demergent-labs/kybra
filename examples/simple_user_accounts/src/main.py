from kybra import query, Record, ic

class User(Record):
    username: str

@query
def get_user() -> User:
    return {
        'username': 'lastmjs'
    }

@query
def print_user(user: User) -> User:
    ic.print(user)
    return user
