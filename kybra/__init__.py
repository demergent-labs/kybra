from typing import Callable, Generator, Generic, NoReturn, Optional, TypedDict, TypeVar, Any, Type, TypeAlias
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

Async = Generator[Any, Any, T]

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

CanisterResult = tuple

# TODO Once RustPython supports Python 3.11, we can use the below and unify CanisterResult with the other Variants
# TODO The problem is that you can't really use generics with TypedDict yet: https://github.com/python/cpython/issues/89026
# TODO We could also consider a hack where we remove all references to CanisterResult before runtime, since this is really an analysis-time consideration
# class CanisterResult(Variant, Generic[T], total=False):
#     ok: T
#     err: str

class RejectionCode(Variant, total=False):
    NoError: None
    SysFatal: None
    SysTransient: None
    DestinationInvalid: None
    CanisterReject: None
    CanisterError: None
    Unknown: None

class NotifyRawResult(Variant, total=False):
    ok: None
    err: RejectionCode

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

class ic(Generic[T]):
    @staticmethod
    def call_raw(canister_id: Principal, method: str, args_raw: blob, payment: nat64) -> CanisterResult[T, str]:
        return {
            'name': 'call_raw',
            'args': [
                canister_id,
                method,
                args_raw,
                payment
            ]
        } # type: ignore

    @staticmethod
    def call_raw128(canister_id: Principal, method: str, args_raw: blob, payment: nat) -> CanisterResult[T, str]:
        return {
            'name': 'call_raw128',
            'args': [
                canister_id,
                method,
                args_raw,
                payment
            ]
        } # type: ignore

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
    ) -> NotifyRawResult:
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
