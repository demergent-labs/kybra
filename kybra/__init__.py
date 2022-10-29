from typing import Callable, NoReturn, Optional, TypeVar, TypedDict, Any, Type, TypeAlias
from .compiler.custom_modules.principal import Principal

Principal = Principal

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

null: TypeAlias = None

reserved = Any
empty = NoReturn # TODO in Python 3.11 I believe there is a Never type

def query(func: object):
    return func

def update(func: object):
    return func

def canister(cls: T) -> T:
    return cls

# TODO need service

Query = Callable
Update = Callable
Oneway = Callable

# TODO make CanisterResult generic
class CanisterResult(Variant, total=False):
    ok: bool
    err: str

# TODO we might want to just make this a dict[int, str] to keep the derive simple, or maybe a class with a principal and method name field
def Func(callable: Callable) -> Type[tuple[Principal, str]]: # type: ignore
    return type((Principal.from_str('aaaaa-aa'), ''))

class ic:
    @staticmethod
    def candid_encode(candid_string: str) -> blob:
        return _kybra_ic.candid_encode(candid_string) # type: ignore

    @staticmethod
    def candid_decode(candid_encoded: blob) -> str:
        return _kybra_ic.candid_decode(candid_encoded) # type: ignore

    @staticmethod
    def notify_raw(
        canister_id: Principal,
        method: str,
        args_raw: blob,
        payment: nat
    ) -> CanisterResult:
        return _kybra_ic.notify_raw(canister_id, method, args_raw, payment) # type: ignore

    @staticmethod
    def print(x: Any):
        _kybra_ic.print(str(x)) # type: ignore
