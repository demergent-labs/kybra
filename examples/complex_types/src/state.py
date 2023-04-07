from kybra import Vec
from src.candid_types import ReactionType
from typing import TypedDict

state: "State" = {"posts": {}, "reactions": {}, "threads": {}, "users": {}}


class State(TypedDict):
    posts: dict[str, "StatePost"]
    reactions: dict[str, "StateReaction"]
    threads: dict[str, "StateThread"]
    users: dict[str, "StateUser"]


class StatePost(TypedDict):
    id: str
    author_id: str
    reaction_ids: Vec[str]
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
    post_ids: Vec[str]
    title: str


class StateUser(TypedDict):
    id: str
    post_ids: Vec[str]
    reaction_ids: Vec[str]
    thread_ids: Vec[str]
    username: str
