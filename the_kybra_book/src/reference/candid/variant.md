# variant

This section is a work in progress.

Python classes that inherit from the Kybra type `Variant` correspond to the [Candid variant type](https://internetcomputer.org/docs/current/references/candid-ref#type-variant--n--t--) and will become [Python TypedDicts](https://docs.python.org/3/library/typing.html#typing.TypedDict) at runtime.

Python:

```python
from kybra import nat32, Variant

class ReactionType(Variant, total=False):
    Fire: None
    ThumbsUp: None
    ThumbsDown: None
    Emotion: 'Emotion'
    Firework: 'Firework'

class Emotion(Variant, total=False):
    Happy: None
    Sad: None

class Firework(Variant, total=False):
    Color: str
    NumStreaks: nat32
```

Candid:

```python
type ReactionType = variant {
    "Fire": null;
    "ThumbsUp": null;
    "ThumbsDown": null;
    "Emotion": Emotion;
    "Firework": Firework
};

type Emotion = variant {
    "Happy": null;
    "Sad": null
};

type Firework = record {
    "Color": text;
    "NumStreaks": nat32;
};
```
