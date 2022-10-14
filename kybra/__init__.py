from typing import Callable, NoReturn, Optional, TypeVar, TypedDict, Any, NoReturn, Type

int64 = int
int32 = int
int16 = int
int8 = int

nat = int
nat64 = int
nat32 = int
nat16 = int
nat8 = int

float64 = float
float32 = float

text = str

T = TypeVar('T')
opt = Optional[T]

Record = TypedDict
Variant = TypedDict

blob = bytes
# blob = bytearray

null = None

reserved = Any
empty = NoReturn # TODO in Python 3.11 I believe there is a Never type

def query(func: object):
    return func

def update(func: object):
    return func

def canister(cls: T) -> T:
    return cls

# TODO need principal
# TODO need service

Query = Callable
Update = Callable
Oneway = Callable

def Func(callable: Callable) -> Type[tuple[int, str]]:
    return type((0, ''))
