import sys
from typing import Any, Callable, Generator, Generic, NoReturn, Optional, ParamSpec, TypedDict, TypeVar, Type, TypeAlias

# TODO I think we can simplify this just like we're doing with canisters
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
manual = Optional[T]

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

class CanisterResult(Generic[T]):
    ok: T
    err: Optional[str]

    def __init__(self, ok: T, err: str):
        self.ok = ok
        self.err = err

    def notify(self) -> "NotifyResult": ...

    def with_cycles(self, cycles: nat64) -> "CanisterResult[T]": ...

    def with_cycles128(self, cycles: nat) -> "CanisterResult[T]": ...

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

# TODO we might want this to act more like CanisterResult
class NotifyResult(Variant, total=False):
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
def Func(callable: Callable[..., Any]) -> Type[tuple[Principal, str]]:
    return type((Principal.from_str('aaaaa-aa'), ''))

def get_first_called_function_name() -> str:
    first_frame = get_first_frame(sys._getframe()) # type: ignore
    return first_frame.f_code.co_name

def get_first_frame(current_frame: Any) -> Any:
    previous_frame = current_frame.f_back

    if previous_frame is None:
        return current_frame

    return get_first_frame(previous_frame)

# TODO make sure call_raw with notify works
class ic(Generic[T]):
    @staticmethod
    def call_raw(canister_id: Principal, method: str, args_raw: blob, payment: nat64) -> CanisterResult[T]:
        return AsyncInfo('call_raw', [
            canister_id,
            method,
            args_raw,
            payment
        ]) # type: ignore

    @staticmethod
    def call_raw128(canister_id: Principal, method: str, args_raw: blob, payment: nat) -> CanisterResult[T]:
        return AsyncInfo('call_raw128', [
            canister_id,
            method,
            args_raw,
            payment
        ]) # type: ignore

    @staticmethod
    def accept_message():
        _kybra_ic.accept_message() #type: ignore

    @staticmethod
    def caller() -> Principal:
        return _kybra_ic.caller() # type: ignore

    @staticmethod
    def candid_encode(candid_string: str) -> blob:
        return _kybra_ic.candid_encode(candid_string) # type: ignore

    @staticmethod
    def candid_decode(candid_encoded: blob) -> str:
        return _kybra_ic.candid_decode(candid_encoded) # type: ignore

    @staticmethod
    def canister_balance() -> nat64:
        return _kybra_ic.canister_balance() # type: ignore

    @staticmethod
    def canister_balance128() -> nat:
        return _kybra_ic.canister_balance128() # type: ignore

    @staticmethod
    def method_name() -> str:
        return _kybra_ic.method_name() #type:ignore

    @staticmethod
    def msg_cycles_accept(max_amount: nat64) -> nat64:
        return _kybra_ic.msg_cycles_accept(max_amount) # type: ignore

    @staticmethod
    def msg_cycles_accept128(max_amount: nat) -> nat:
        return _kybra_ic.msg_cycles_accept128(max_amount) # type: ignore

    @staticmethod
    def msg_cycles_available() -> nat64:
        return _kybra_ic.msg_cycles_available() # type: ignore

    @staticmethod
    def msg_cycles_available128() -> nat:
        return _kybra_ic.msg_cycles_available128() # type: ignore

    @staticmethod
    def msg_cycles_refunded() -> nat64:
        return _kybra_ic.msg_cycles_refunded() # type: ignore

    @staticmethod
    def msg_cycles_refunded128() -> nat:
        return _kybra_ic.msg_cycles_refunded128() # type: ignore

    @staticmethod
    def notify_raw(
        canister_id: Principal,
        method: str,
        args_raw: blob,
        payment: nat
    ) -> NotifyResult:
        return _kybra_ic.notify_raw(canister_id, method, args_raw, payment) # type: ignore

    @staticmethod
    def performance_counter(counter_type: nat32) -> nat64:
        return _kybra_ic.performance_counter(counter_type) # type: ignore

    @staticmethod
    def print(x: Any):
        _kybra_ic.print(str(x)) # type: ignore

    @staticmethod
    def reject(x: Any):
        _kybra_ic.reject(x) # type: ignore

    @staticmethod
    def reply(value: Any):
        first_called_function_name = get_first_called_function_name()
        _kybra_ic.reply(first_called_function_name, value) # type: ignore

    @staticmethod
    def reply_raw(x: Any):
        _kybra_ic.reply_raw(x) # type: ignore

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

    @staticmethod
    def stable_storage() -> Any:
        return _kybra_stable_storage # type: ignore

    @staticmethod
    def time() -> nat64:
        return _kybra_ic.time() # type: ignore

    @staticmethod
    def trap(message: str) -> NoReturn: # type: ignore
        _kybra_ic.trap(message) # type: ignore

class Canister:
    canister_id: Principal

    def __init__(self, canister_id: Principal):
        self.canister_id = canister_id

P = ParamSpec('P')

class AsyncInfo:
    name: str
    args: list[Any]

    def __init__(self, name: str, args: list[Any]):
        self.name = name
        self.args = args

    def with_cycles(self, cycles: nat64) -> "AsyncInfo":
        return AsyncInfo('call_with_payment', [*self.args, cycles])

    def with_cycles128(self, cycles: nat) -> "AsyncInfo":
        return AsyncInfo('call_with_payment128', [*self.args, cycles])

    def notify(self) -> NotifyResult:
        qualname: str = self.args[1]
        with_payment = 'with_payment128_' if self.name == 'call_with_payment' or self.name == 'call_with_payment128' else ''
        notify_function_name = f'_azle_notify_{with_payment}{qualname.replace(".", "_")}_wrapper'

        return getattr(_kybra_ic, notify_function_name)(self.args) # type: ignore

# TODO this decorator is removing the static type checking of the self parameter for instance methods
# TODO watch out for *kwargs
def method(func: Callable[P, T]) -> Callable[P, CanisterResult[T]]:
    def intermediate_func(*args): # type: ignore
        the_self = args[0] # type: ignore
        selfless_args = args[1:] # type: ignore

        return AsyncInfo('call', [
            the_self.canister_id, # type: ignore
            func.__qualname__,
            *selfless_args
        ])

    return intermediate_func # type: ignore
