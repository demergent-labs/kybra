# record

This section is a work in progress.

Python classes that inherit from the Kybra type `Record` correspond to the [Candid record type](https://internetcomputer.org/docs/current/references/candid-ref#type-record--n--t--) and will become [Python TypedDicts](https://docs.python.org/3/library/typing.html#typing.TypedDict) at runtime.

Python:

```python
from kybra import Record

class Post(Record):
    id: str
    author: 'User'
    text: str
    thread: 'Thread'

class Thread(Record):
    id: str
    author: 'User'
    posts: list[Post]
    title: str

class User(Record):
    id: str
    posts: list[Post]
    thread: list[Thread]
    username: str
```

Candid:

```python
type Post = record {
    "id": text;
    "author": User;
    "text": text;
    "thread": Thread;
};

type Thread = record {
    "id": text;
    "author": User;
    "posts": vec Post;
    "title": text;
};

type User = record {
    "id": text;
    "posts": vec Post;
    "threads": vec Thread;
    "username": text;
};
```
