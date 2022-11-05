from src.candid_types import ReactionType
from typing import TypedDict

state: "State" = {
    'posts': {},
    'reactions': {},
    'threads': {},
    'users': {}
}

class State(TypedDict):
    posts: dict[str, "StatePost"]
    reactions: dict[str, "StateReaction"]
    threads: dict[str, "StateThread"]
    users: dict[str, "StateUser"]

class StatePost(TypedDict):
    id: str
    author_id: str
    reaction_ids: list[str]
    text: str
    thread_id: str

class StateReaction(TypedDict):
    id: str
    author_id: str
    post_id: str
    reaction_type: ReactionType

class StateThread(TypedDict):
    id: str
    author_id: str
    post_ids: list[str]
    title: str

class StateUser(TypedDict):
    id: str
    post_ids: list[str]
    reaction_ids: list[str]
    thread_ids: list[str]
    username: str
