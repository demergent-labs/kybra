# Candid

-   [text](#text)
-   [blob](#blob)
-   [nat](#nat)
-   [nat64](#nat64)
-   [nat32](#nat32)
-   [nat16](#nat16)
-   [nat8](#nat8)
-   [int](#int)
-   [int64](#int64)
-   [int32](#int32)
-   [int16](#int16)
-   [int8](#int8)
-   [float64](#float64)
-   [float32](#float32)
-   [bool](#bool)
-   [null](#null)
-   [vec](#vec)
-   [opt](#opt)
-   [record](#record)
-   [variant](#variant)
-   [func](#func)
-   [service](#service)
-   [principal](#principal)
-   [reserved](#reserved)
-   [empty](#empty)

[Candid](https://internetcomputer.org/docs/current/developer-docs/backend/candid/) is an interface description language created by [DFINITY](https://dfinity.org/). It can be used to define interfaces between services (canisters), allowing canisters and clients written in various languages to easily interact with each other.

Kybra allows you to express Candid types through a combination of native and Kybra-provided Python types. These types will be necessary in various places as you define your canister. For example, Candid types must be used when defining the parameters and return types of your query and update methods.

It's important to note that the Candid types are represented at runtime using specific Python data structures that may differ in behavior from the description of the actual Candid type. For example, a `float32` Candid type is a [Python float](https://docs.python.org/3/library/functions.html#float), a `nat64` is a [Python int](https://docs.python.org/3/library/functions.html#int), and an `int` is also a [Python int](https://docs.python.org/3/library/functions.html#int).

Keep this in mind as it may result in unexpected behavior. Each Candid type and its equivalent Python runtime value is explained in more detail in this chapter.

A reference of all Candid types available on the Internet Computer (IC) can be found [here](https://internetcomputer.org/docs/current/references/candid-ref).

The following is a simple example showing how to import and use most of the Candid types available in Kybra:

```python
from kybra import (
    blob,
    float64,
    float32,
    Func,
    int8,
    int16,
    int32,
    int64,
    nat,
    nat8,
    nat16,
    nat32,
    nat64,
    null,
    opt,
    Principal,
    query,
    Query,
    Record,
    Variant,
)


class Candid(Record):
    text: str
    blob: blob
    nat: nat
    nat64: nat64
    nat32: nat32
    nat16: nat16
    nat8: nat8
    int: int
    int64: int64
    int32: int32
    int16: int16
    int8: int8
    float64: float64
    float32: float32
    bool: bool
    opt: opt["nat"]
    record: "CandidRecord"
    variant: "CandidVariant"
    func: "CandidFunc"
    principal: Principal


class CandidRecord(Record):
    first_name: str
    last_name: str
    age: nat8


class CandidVariant(Variant, total=False):
    Tag1: null
    Tag2: null
    Tag3: int


CandidFunc = Func(Query[[], Candid])


@query
def candid_types() -> Candid:
    return {
        "text": "text",
        "blob": bytes(),
        "nat": 340_282_366_920_938_463_463_374_607_431_768_211_455,
        "nat64": 18_446_744_073_709_551_615,
        "nat32": 4_294_967_295,
        "nat16": 65_535,
        "nat8": 255,
        "int": 170_141_183_460_469_231_731_687_303_715_884_105_727,
        "int64": 9_223_372_036_854_775_807,
        "int32": 2_147_483_647,
        "int16": 32_767,
        "int8": 127,
        "float64": 0.0,
        "float32": 0.0,
        "bool": True,
        "opt": None,
        "record": {"first_name": "John", "last_name": "Doe", "age": 35},
        "variant": {"Tag1": None},
        "func": (Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai"), "candid_types"),
        "principal": Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"),
    }
```

Calling `candid_types` with `dfx` will return:

```
(
  record {
    "int" = 170_141_183_460_469_231_731_687_303_715_884_105_727 : int;
    "nat" = 340_282_366_920_938_463_463_374_607_431_768_211_455 : nat;
    "opt" = null;
    "principal" = principal "ryjl3-tyaaa-aaaaa-aaaba-cai";
    "blob" = vec {};
    "bool" = true;
    "func" = func "rrkah-fqaaa-aaaaa-aaaaq-cai".candid_types;
    "int8" = 127 : int8;
    "nat8" = 255 : nat8;
    "text" = "text";
    "nat16" = 65_535 : nat16;
    "nat32" = 4_294_967_295 : nat32;
    "nat64" = 18_446_744_073_709_551_615 : nat64;
    "int16" = 32_767 : int16;
    "int32" = 2_147_483_647 : int32;
    "int64" = 9_223_372_036_854_775_807 : int64;
    "variant" = variant { Tag1 };
    "float32" = 0 : float32;
    "float64" = 0 : float64;
    "record" = record {
      age = 35 : nat8;
      first_name = "John";
      last_name = "Doe";
    };
  },
)
```

#### text

The Python type `str` and the Kybra type `text` both correspond to the [Candid type text](https://internetcomputer.org/docs/current/references/candid-ref#type-text) and will become a [Python str](https://docs.python.org/3/library/stdtypes.html#textseq) at runtime.

Python:

```python
from kybra import ic, query


@query
def get_string() -> str:
    return "Hello world!"


@query
def print_string(string: str) -> str:
    ic.print(type(string))
    return string
```

Candid:

```python
service: {
    "get_string": () -> (text) query;
    "print_string": (text) -> (text) query;
}
```

#### blob

The Kybra type `blob` corresponds to the [Candid type blob](https://internetcomputer.org/docs/current/references/candid-ref#type-blob) and will become a [Python bytes](https://docs.python.org/3/library/stdtypes.html#bytes) at runtime.

Python:

```Python
from kybra import blob, ic, query


@query
def get_blob() -> blob:
    return bytes([68, 73, 68, 76, 0, 0])


@query
def print_blob(blob: blob) -> blob:
    ic.print(type(blob))
    return blob
```

Candid:

```
service: {
    "get_blob": () -> (blob) query;
    "print_blob": (blob) -> (blob) query;
}
```

#### nat

The Kybra type `nat` corresponds to the [Candid type nat](https://internetcomputer.org/docs/current/references/candid-ref#type-nat) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat, query


@query
def get_nat() -> nat:
    return 340_282_366_920_938_463_463_374_607_431_768_211_455


@query
def print_nat(nat: nat) -> nat:
    ic.print(type(nat))
    return nat
```

Candid:

```
service: {
    "get_nat": () -> (nat) query;
    "print_nat": (nat) -> (nat) query;
}
```

#### nat64

The Kybra type `nat64` corresponds to the [Candid type nat64](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat64, query


@query
def get_nat64() -> nat64:
    return 18_446_744_073_709_551_615


@query
def print_nat64(nat64: nat64) -> nat64:
    ic.print(type(nat64))
    return nat64
```

Candid:

```
service: {
    "get_nat64": () -> (nat64) query;
    "print_nat64": (nat64) -> (nat64) query;
}
```

#### nat32

The Kybra type `nat32` corresponds to the [Candid type nat32](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat32, query


@query
def get_nat32() -> nat32:
    return 4_294_967_295


@query
def print_nat32(nat32: nat32) -> nat32:
    ic.print(type(nat32))
    return nat32
```

Candid:

```
service: {
    "get_nat32": () -> (nat32) query;
    "print_nat32": (nat32) -> (nat32) query;
}
```

#### nat16

The Kybra type `nat16` corresponds to the [Candid type nat16](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat16, query


@query
def get_nat16() -> nat16:
    return 65_535


@query
def print_nat16(nat16: nat16) -> nat16:
    ic.print(type(nat16))
    return nat16
```

Candid:

```
service: {
    "get_nat16": () -> (nat16) query;
    "print_nat16": (nat16) -> (nat16) query;
}
```

#### nat8

The Kybra type `nat8` corresponds to the [Candid type nat8](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, nat8, query


@query
def get_nat8() -> nat8:
    return 255


@query
def print_nat8(nat8: nat8) -> nat8:
    ic.print(type(nat8))
    return nat8
```

Candid:

```
service: {
    "get_nat8": () -> (nat8) query;
    "print_nat8": (nat8) -> (nat8) query;
}
```

#### int

The Kybra type `int` corresponds to the [Candid type int](https://internetcomputer.org/docs/current/references/candid-ref#type-int) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, query


@query
def get_int() -> int:
    return 170_141_183_460_469_231_731_687_303_715_884_105_727


@query
def print_int(int: int) -> int:
    ic.print(type(int))
    return int
```

Candid:

```
service: {
    "get_int": () -> (int) query;
    "print_int": (int) -> (int) query;
}
```

#### int64

The Kybra type `int64` corresponds to the [Candid type int64](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int64, query


@query
def get_int64() -> int64:
    return 9_223_372_036_854_775_807


@query
def print_int64(int64: int64) -> int64:
    ic.print(type(int64))
    return int64
```

Candid:

```
service: {
    "get_int64": () -> (int64) query;
    "print_int64": (int64) -> (int64) query;
}
```

#### int32

The Kybra type `int32` corresponds to the [Candid type int32](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int32, query


@query
def get_int32() -> int32:
    return 2_147_483_647


@query
def print_int32(int32: int32) -> int32:
    ic.print(type(int32))
    return int32
```

Candid:

```
service: {
    "get_int32": () -> (int32) query;
    "print_int32": (int32) -> (int32) query;
}
```

#### int16

The Kybra type `int16` corresponds to the [Candid type int16](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int16, query


@query
def get_int16() -> int16:
    return 32_767


@query
def print_int16(int16: int16) -> int16:
    ic.print(type(int16))
    return int16
```

Candid:

```
service: {
    "get_int16": () -> (int16) query;
    "print_int16": (int16) -> (int16) query;
}
```

#### int8

The Kybra type `int8` corresponds to the [Candid type int8](https://internetcomputer.org/docs/current/references/candid-ref#type-natn-and-intn) and will become a [Python int](https://docs.python.org/3/library/functions.html#int) at runtime.

Python:

```python
from kybra import ic, int8, query


@query
def get_int8() -> int8:
    return 127


@query
def print_int8(int8: int8) -> int8:
    ic.print(type(int8))
    return int8
```

Candid:

```
service: {
    "get_int8": () -> (int8) query;
    "print_int8": (int8) -> (int8) query;
}
```

#### float64

The Kybra type `float64` corresponds to the [Candid type float64](https://internetcomputer.org/docs/current/references/candid-ref#type-float32-and-float64) and will become a [Python float](https://docs.python.org/3/library/functions.html#float) at runtime.

Python:

```python
import math

from kybra import float64, ic, query


@query
def get_float64() -> float64:
    return math.e


@query
def print_float64(float64: float64) -> float64:
    ic.print(type(float64))
    return float64
```

Candid:

```
service: {
    "get_float64": () -> (float64) query;
    "print_float64": (float64) -> (float64) query;
}
```

#### float32

The Kybra type `float32` corresponds to the [Candid type float32](https://internetcomputer.org/docs/current/references/candid-ref#type-float32-and-float64) and will become a [Python float](https://docs.python.org/3/library/functions.html#float) at runtime.

Python:

```python
import math

from kybra import float32, ic, query


@query
def get_float32() -> float32:
    return math.pi


@query
def print_float32(float32: float32) -> float32:
    ic.print(type(float32))
    return float32
```

Candid:

```
service: {
    "get_float32": () -> (float32) query;
    "print_float32": (float32) -> (float32) query;
}
```

#### bool

The Python type `bool` corresponds to the [Candid type bool](https://internetcomputer.org/docs/current/references/candid-ref#type-bool) and will become a [Python Boolean Value](https://docs.python.org/3/library/stdtypes.html#boolean-values) at runtime.

Python:

```Python
from kybra import ic, query


@query
def get_bool() -> bool:
    return True


@query
def print_bool(bool: bool) -> bool:
    ic.print(type(bool))
    return bool
```

Candid:

```
service: {
    "get_bool": () -> (bool) query;
    "print_bool": (bool) -> (bool) query;
}
```

#### null

The Python type `None` and the Kybra type `null` both correspond to the [Candid type null](https://internetcomputer.org/docs/current/references/candid-ref#type-null) and will become the [Python Null Object](https://docs.python.org/3/library/stdtypes.html#the-null-object) at runtime.

Python:

```python
from kybra import ic, query


@query
def get_null() -> None:
    return None


@query
def print_null(none: None) -> None:
    ic.print(type(none))
    return none
```

Candid:

```
service: {
    "get_null": () -> (null) query;
    "print_null": (null) -> (null) query;
}
```

#### vec

The Python type `list` corresponds to the [Candid type vec](https://internetcomputer.org/docs/current/references/candid-ref#type-vec-t) and will become an array of the specified type at runtime.

Python:

```python
from kybra import int32, query


@query
def get_numbers() -> list[int32]:
    return [0, 1, 2, 3]
```

Candid:

```
service: {
    "get_numbers": () -> (vec int32) query;
}
```

#### opt

The Kybra type `opt` corresponds to the [Candid type opt](https://internetcomputer.org/docs/current/references/candid-ref#type-opt-t) and will become the enclosed Python type or None at runtime.

Python:

```python
from kybra import opt, query


@query
def get_opt_some() -> opt[bool]:
    return True


@query
def get_opt_none() -> opt[bool]:
    return None
```

Candid:

```
service: {
    "get_opt_some": () -> (opt bool) query;
    "get_opt_none": () -> (opt bool) query;
}
```

#### record

Python classes that inherit from the Kybra type `Record` correspond to the [Candid record type](https://internetcomputer.org/docs/current/references/candid-ref#type-record--n--t--) and will become [Python TypedDicts](https://docs.python.org/3/library/typing.html#typing.TypedDict) at runtime.

Python:

```python
from kybra import Record


class Post(Record):
    id: str
    author: "User"
    text: str
    thread: "Thread"


class Thread(Record):
    id: str
    author: "User"
    posts: list[Post]
    title: str


class User(Record):
    id: str
    posts: list[Post]
    thread: list[Thread]
    username: str
```

Candid:

```
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

#### variant

Python classes that inherit from the Kybra type `Variant` correspond to the [Candid variant type](https://internetcomputer.org/docs/current/references/candid-ref#type-variant--n--t--) and will become [Python TypedDicts](https://docs.python.org/3/library/typing.html#typing.TypedDict) at runtime.

Python:

```python
from kybra import nat32, Variant


class ReactionType(Variant, total=False):
    Fire: None
    ThumbsUp: None
    ThumbsDown: None
    Emotion: "Emotion"
    Firework: "Firework"


class Emotion(Variant, total=False):
    Happy: None
    Sad: None


class Firework(Variant, total=False):
    Color: str
    NumStreaks: nat32
```

Candid:

```
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

#### func

The Kybra type `Func` corresponds to the [Candid type func](https://internetcomputer.org/docs/current/references/candid-ref#type-func---) and at runtime will become a Python tuple with two elements, the first being an [ic-py Principal](https://github.com/rocklabs-io/ic-py) and the second being a [Python str](https://docs.python.org/3/library/stdtypes.html#textseq). The `ic-py Principal` represents the `principal` of the canister/service where the function exists, and the `str` represents the function's name.

Python:

```python
from kybra import Func, nat64, null, Principal, query, Query, Record, Update, Variant


class User(Record):
    id: str
    basic_func: "BasicFunc"
    complex_func: "ComplexFunc"


class Reaction(Variant, total=False):
    Good: null
    Bad: null
    BasicFunc: "BasicFunc"
    ComplexFunc: "ComplexFunc"


BasicFunc = Func(Query[[str], str])
ComplexFunc = Func(Update[[User, Reaction], nat64])


@query
def get_basic_func() -> BasicFunc:
    return (Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai"), "simple_function_name")


@query
def get_complex_func() -> ComplexFunc:
    return (Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"), "complex_function_name")
```

Candid:

```
type User = record {
    "id": text;
    "basic_func": BasicFunc;
    "complex_func": ComplexFunc;
};
type Reaction = variant { "Good": null; "Bad": null; "BasicFunc": BasicFunc; "ComplexFunc": ComplexFunc };

type BasicFunc = func (text) -> (text) query;
type ComplexFunc = func (User, Reaction) -> (nat64);

service: () -> {
    "get_basic_func": () -> (BasicFunc) query;
    "get_complex_func": () -> (ComplexFunc) query;
}

```

#### service

[Not yet implemented.](https://github.com/demergent-labs/kybra/issues/124)

#### principal

The Kybra type `Principal` corresponds to the [Candid type principal](https://internetcomputer.org/docs/current/references/candid-ref#type-principal) and will become an [ic-py Principal](https://github.com/rocklabs-io/ic-py) at runtime.

Python:

```python
from kybra import ic, Principal, query


@query
def get_principal() -> Principal:
    return Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai")


@query
def print_principal(principal: Principal) -> Principal:
    ic.print(type(principal))
    return principal
```

Candid:

```
service: {
    "get_principal": () -> (principal) query;
    "print_principal": (principal) -> (principal) query;
}
```

#### reserved

The Kybra type `reserved` corresponds to the [Candid type reserved](https://internetcomputer.org/docs/current/references/candid-ref#type-reserved) and will become the [Python Null Object](https://docs.python.org/3/library/stdtypes.html#the-null-object) at runtime.

Python:

```python
from kybra import ic, query, reserved


@query
def get_reserved() -> reserved:
    return "anything"


@query
def print_reserved(reserved: reserved) -> reserved:
    ic.print(type(reserved))
    return reserved
```

Candid:

```
service: {
    "get_reserved": () -> (reserved) query;
    "print_reserved": (reserved) -> (reserved) query;
}
```

#### empty

The Kybra type `empty` corresponds to the [Candid type empty](https://internetcomputer.org/docs/current/references/candid-ref#type-empty) and has no Python value at runtime.

Python:

```python
from kybra import empty, ic, query


def get_empty() -> empty:
    raise Exception("Anything you want")


# Note: It is impossible to call this function because it requires an argument
# but there is no way to pass an "empty" value as an argument.
@query
def print_empty(empty: empty) -> empty:
    ic.print(type(empty))
    raise Exception("Anything you want")
```

Candid:

```
service: {
    "get_empty": () -> (empty) query;
    "print_empty": (empty) -> (empty) query;
}
```
