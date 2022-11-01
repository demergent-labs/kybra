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

def init(func: object):
    return func

def heartbeat(func: object):
    return func

def pre_upgrade(func: object):
    return func

def post_upgrade(func: object):
    return func

def inspect_message(func: object):
    return func

# TODO need service

Query = Callable
Update = Callable
Oneway = Callable

# TODO make CanisterResult generic
class CanisterResult(Variant, total=False):
    ok: bool
    err: str

# TODO remove temp on StableMemoryError, StableGrowResult, Stable64GrowResult once multiple file analysis works
class StableMemoryErrorTemp(Variant, total=False):
    OutOfMemory: None
    OutOfBounds: None

class StableGrowResultTemp(Variant, total=False):
    ok: nat32
    err: StableMemoryErrorTemp

class Stable64GrowResultTemp(Variant, total=False):
    ok: nat64
    err: StableMemoryErrorTemp

# TODO we might want to just make this a dict[int, str] to keep the derive simple, or maybe a class with a principal and method name field
def Func(callable: Callable) -> Type[tuple[Principal, str]]: # type: ignore
    return type((Principal.from_str('aaaaa-aa'), ''))

class ic:
    @staticmethod
    def accept_message():
        _kybra_ic.accept_message() #type:ignore

    @staticmethod
    def candid_encode(candid_string: str) -> blob:
        return _kybra_ic.candid_encode(candid_string) # type: ignore

    @staticmethod
    def candid_decode(candid_encoded: blob) -> str:
        return _kybra_ic.candid_decode(candid_encoded) # type: ignore

    @staticmethod
    def method_name() -> str:
        return _kybra_ic.method_name() #type:ignore

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

    @staticmethod
    def stable_bytes() -> blob:
        return _kybra_ic.stable_bytes() # type: ignore

    @staticmethod
    def stable_grow(new_pages: nat32) -> StableGrowResultTemp:
        return _kybra_ic.stable_grow(new_pages) # type: ignore

    @staticmethod
    def stable_read(offset: nat32, length: nat32) -> blob:
        return _kybra_ic.stable_read(offset, length) # type: ignore

    @staticmethod
    def stable_size() -> nat32:
        return _kybra_ic.stable_size() # type: ignore

    @staticmethod
    def stable_write(offset: nat32, buf: blob):
        _kybra_ic.stable_write(offset, buf) # type: ignore

    @staticmethod
    def stable64_grow(new_pages: nat64) -> Stable64GrowResultTemp:
        return _kybra_ic.stable64_grow(new_pages) # type: ignore

    @staticmethod
    def stable64_read(offset: nat64, length: nat64) -> blob:
        return _kybra_ic.stable64_read(offset, length) # type: ignore

    @staticmethod
    def stable64_size() -> nat64:
        return _kybra_ic.stable64_size() # type: ignore

    @staticmethod
    def stable64_write(offset: nat64, buf: blob):
        _kybra_ic.stable64_write(offset, buf) # type: ignore
