from kybra import init, null, opt, Principal, query, Record, Variant


class User(Record):
    id: str


class Reaction(Variant, total=False):
    Fire: null
    Wave: null


user: opt[User] = None
reaction: opt[Reaction] = None
owner: opt[Principal] = None


@init
def init_(init_user: User, init_reaction: Reaction, init_owner: Principal):
    global user
    global reaction
    global owner

    user = init_user
    reaction = init_reaction
    owner = init_owner


@query
def get_user() -> opt[User]:
    return user


@query
def get_reaction() -> opt[Reaction]:
    return reaction


@query
def get_owner() -> opt[Principal]:
    return owner
