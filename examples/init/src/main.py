from kybra import init, null, Opt, Principal, query, Record, Variant, void


class User(Record):
    id: str


class Reaction(Variant, total=False):
    Fire: null
    Wave: null


user: Opt[User] = None
reaction: Opt[Reaction] = None
owner: Opt[Principal] = None


@init
def init_(init_user: User, init_reaction: Reaction, init_owner: Principal) -> void:
    global user
    global reaction
    global owner

    user = init_user
    reaction = init_reaction
    owner = init_owner


@query
def get_user() -> Opt[User]:
    return user


@query
def get_reaction() -> Opt[Reaction]:
    return reaction


@query
def get_owner() -> Opt[Principal]:
    return owner
