from kybra import Opt, query, Record, update, Vec
from typing import TypedDict


class User(Record):
    id: str
    username: str


class Db(TypedDict):
    users: dict[str, User]


db: Db = {"users": {}}


@query
def get_user_by_id(id: str) -> Opt[User]:
    user = db["users"].get(id, None)

    return user


@query
def get_all_users() -> Vec[User]:
    return list(db["users"].values())


@update
def create_user(username: str) -> User:
    id = str(len(db["users"].keys()))

    user: User = {"id": id, "username": username}

    db["users"][id] = user

    return user
