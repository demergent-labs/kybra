from kybra import query, Record, Variant, ic

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

class Reaction(Variant, total=False):
    Happy: None
    Sad: str

@query
def get_reaction() -> Reaction:
    return {
        'Sad': 'yous'
    }

@query
def print_reaction(reaction: Reaction) -> bool:
    ic.print(reaction)
    return True
