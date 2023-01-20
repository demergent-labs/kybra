from kybra import null, Record, Variant


class Post(Record):
    id: str
    author: "User"
    reactions: list["Reaction"]
    text: str
    thread: "Thread"


class Reaction(Record):
    id: str
    author: "User"
    post: Post
    reaction_type: "ReactionType"


class ReactionType(Variant):
    Fire: null
    ThumbsUp: null
    ThumbsDown: null


class Thread(Record):
    id: str
    author: "User"
    posts: list[Post]
    title: str


class User(Record):
    id: str
    posts: list[Post]
    reactions: list[Reaction]
    threads: list[Thread]
    username: str
