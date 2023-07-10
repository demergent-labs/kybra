from kybra import null, Record, Variant, Vec


class Post(Record):
    id: str
    author: "User"
    reactions: Vec["Reaction"]
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
    posts: Vec[Post]
    title: str


class User(Record):
    id: str
    posts: Vec[Post]
    reactions: Vec[Reaction]
    threads: Vec[Thread]
    username: str
