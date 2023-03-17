import sys
from typing import (
    Annotated,
    Any,
    Callable,
    Generator,
    Generic,
    NoReturn,
    Optional,
    ParamSpec,
    TypedDict,
    TypeVar,
    TypeAlias,
)

# TODO I think we can simplify this just like we're doing with canisters
from .compiler.custom_modules.principal import Principal as PrincipalRenamed

__version__ = "0.4.0"
__rust_version__ = "1.66.0"

Principal = PrincipalRenamed

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

T = TypeVar("T")
opt = Optional[T]
manual = Optional[T]
alias = Annotated[T, None]


Record = TypedDict
Variant = TypedDict

blob = bytes

null: TypeAlias = None
void: TypeAlias = None

reserved = Any
empty: TypeAlias = NoReturn  # TODO in Python 3.11 I believe there is a Never type

Async = Generator[Any, Any, T]

TimerId = alias[nat64]
Duration = alias[nat64]


GuardType = Callable[..., Any]


def query(
    _func: Optional[Callable[..., Any]] = None, *, guard: Optional[GuardType] = None
) -> Callable[..., Any]:
    def decorator(func: Callable[..., Any]):
        return func

    if _func is None:
        return decorator
    else:
        return decorator(_func)


def update(
    _func: Optional[Callable[..., Any]] = None, *, guard: Optional[GuardType] = None
) -> Callable[..., Any]:
    def decorator(func: Callable[..., Any]):
        return func

    if _func is None:
        return decorator
    else:
        return decorator(_func)


def canister(cls: T) -> T:
    return cls


def init(func: object):
    return func


def heartbeat(
    _func: Optional[Callable[..., Any]] = None, *, guard: Optional[GuardType] = None
) -> Callable[..., Any]:
    def decorator(func: Callable[..., Any]):
        return func

    if _func is None:
        return decorator
    else:
        return decorator(_func)


def pre_upgrade(
    _func: Optional[Callable[..., Any]] = None, *, guard: Optional[GuardType] = None
) -> Callable[..., Any]:
    def decorator(func: Callable[..., Any]):
        return func

    if _func is None:
        return decorator
    else:
        return decorator(_func)


def post_upgrade(func: object):
    return func


def inspect_message(
    _func: Optional[Callable[..., Any]] = None, *, guard: Optional[GuardType] = None
) -> Callable[..., Any]:
    def decorator(func: Callable[..., Any]):
        return func

    if _func is None:
        return decorator
    else:
        return decorator(_func)


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

    def notify(self) -> "NotifyResult":
        ...

    def with_cycles(self, cycles: nat64) -> "CanisterResult[T]":
        ...

    def with_cycles128(self, cycles: nat) -> "CanisterResult[T]":
        ...


# TODO Once RustPython supports Python 3.11, we can use the below and unify CanisterResult with the other Variants
# TODO The problem is that you can't really use generics with TypedDict yet: https://github.com/python/cpython/issues/89026
# TODO We could also consider a hack where we remove all references to CanisterResult before runtime, since this is really an analysis-time consideration
# class CanisterResult(Variant, Generic[T], total=False):
#     ok: T
#     err: str


class GuardResult(Variant, total=False):
    ok: null
    err: str


class RejectionCode(Variant, total=False):
    NoError: null
    SysFatal: null
    SysTransient: null
    DestinationInvalid: null
    CanisterReject: null
    CanisterError: null
    Unknown: null


# TODO we might want this to act more like CanisterResult
class NotifyResult(Variant, total=False):
    ok: null
    err: RejectionCode


class StableMemoryError(Variant, total=False):
    OutOfMemory: null
    OutOfBounds: null


class StableGrowResult(Variant, total=False):
    ok: nat32
    err: StableMemoryError


class Stable64GrowResult(Variant, total=False):
    ok: nat64
    err: StableMemoryError


FuncTuple = tuple[Principal, str]


def Func(callable: Callable[..., Any]) -> type[FuncTuple]:
    return FuncTuple


def get_first_called_function_name() -> str:
    first_frame = get_first_frame(sys._getframe())  # type: ignore
    return first_frame.f_code.co_name


def get_first_frame(current_frame: Any) -> Any:
    previous_frame = current_frame.f_back

    if previous_frame is None:
        return current_frame

    return get_first_frame(previous_frame)


class ic(Generic[T]):
    @ staticmethod
    def accept_message():
        _kybra_ic._kybra_accept_message()  # type: ignore

    @ staticmethod
    def arg_data_raw() -> blob:
        return _kybra_ic._kybra_arg_data_raw()  # type: ignore

    @ staticmethod
    def arg_data_raw_size() -> nat32:
        return _kybra_ic._kybra_arg_data_raw_size()  # type: ignore

    @ staticmethod
    def call_raw(
        canister_id: Principal, method: str, args_raw: blob, payment: nat64
    ) -> CanisterResult[T]:
        return AsyncInfo(
            "call_raw", [canister_id, method, args_raw, payment]
        )  # type: ignore

    @staticmethod
    def call_raw128(
        canister_id: Principal, method: str, args_raw: blob, payment: nat
    ) -> CanisterResult[T]:
        return AsyncInfo(
            "call_raw128", [canister_id, method, args_raw, payment]
        )  # type: ignore

    @staticmethod
    def caller() -> Principal:
        return _kybra_ic._kybra_caller()  # type: ignore

    @staticmethod
    def candid_encode(candid_string: str) -> blob:
        return _kybra_ic._kybra_candid_encode(candid_string)  # type: ignore

    @staticmethod
    def candid_decode(candid_encoded: blob) -> str:
        return _kybra_ic._kybra_candid_decode(candid_encoded)  # type: ignore

    @staticmethod
    def canister_balance() -> nat64:
        return _kybra_ic._kybra_canister_balance()  # type: ignore

    @staticmethod
    def canister_balance128() -> nat:
        return _kybra_ic._kybra_canister_balance128()  # type: ignore

    @staticmethod
    def clear_timer(id: TimerId) -> None:
        return _kybra_ic._kybra_clear_timer(id)  # type: ignore

    @staticmethod
    def data_certificate() -> opt[blob]:
        return _kybra_ic._kybra_data_certificate()  # type: ignore

    @staticmethod
    def id() -> Principal:
        return _kybra_ic._kybra_id()  # type:ignore

    @staticmethod
    def method_name() -> str:
        return _kybra_ic._kybra_method_name()  # type:ignore

    @staticmethod
    def msg_cycles_accept(max_amount: nat64) -> nat64:
        return _kybra_ic._kybra_msg_cycles_accept(max_amount)  # type: ignore

    @staticmethod
    def msg_cycles_accept128(max_amount: nat) -> nat:
        return (
            _kybra_ic._kybra_msg_cycles_accept128(max_amount)  # type: ignore
        )

    @staticmethod
    def msg_cycles_available() -> nat64:
        return _kybra_ic._kybra_msg_cycles_available()  # type: ignore

    @staticmethod
    def msg_cycles_available128() -> nat:
        return _kybra_ic._kybra_msg_cycles_available128()  # type: ignore

    @staticmethod
    def msg_cycles_refunded() -> nat64:
        return _kybra_ic._kybra_msg_cycles_refunded()  # type: ignore

    @staticmethod
    def msg_cycles_refunded128() -> nat:
        return _kybra_ic._kybra_msg_cycles_refunded128()  # type: ignore

    @staticmethod
    def notify_raw(
        canister_id: Principal, method: str, args_raw: blob, payment: nat
    ) -> NotifyResult:
        return _kybra_ic._kybra_notify_raw(  # type: ignore
            canister_id, method, args_raw, payment
        )

    @staticmethod
    def performance_counter(counter_type: nat32) -> nat64:
        return (
            _kybra_ic._kybra_performance_counter(counter_type)  # type: ignore
        )

    @staticmethod
    def print(x: Any):
        _kybra_ic._kybra_print(str(x))  # type: ignore

    @staticmethod
    def reject(x: Any):
        _kybra_ic._kybra_reject(x)  # type: ignore

    @staticmethod
    def reject_code() -> RejectionCode:
        return _kybra_ic._kybra_reject_code()  # type: ignore

    @staticmethod
    def reject_message() -> str:
        return _kybra_ic._kybra_reject_message()  # type: ignore

    @staticmethod
    def reply(value: Any):
        first_called_function_name = get_first_called_function_name()
        (
            _kybra_ic._kybra_reply(  # type: ignore
                first_called_function_name, value)
        )

    @staticmethod
    def reply_raw(x: Any):
        _kybra_ic._kybra_reply_raw(x)  # type: ignore

    @staticmethod
    def set_certified_data(data: blob):
        _kybra_ic._kybra_set_certified_data(data)  # type: ignore

    @staticmethod
    def set_timer(delay: Duration, func: Callable[[], Any]) -> TimerId:
        return _kybra_ic._kybra_set_timer(delay, func)  # type: ignore

    @staticmethod
    def set_timer_interval(interval: Duration, func: Callable[[], Any]) -> TimerId:
        return (
            _kybra_ic._kybra_set_timer_interval(interval, func)  # type: ignore
        )

    @staticmethod
    def stable_bytes() -> blob:
        return _kybra_ic._kybra_stable_bytes()  # type: ignore

    @staticmethod
    def stable_grow(new_pages: nat32) -> StableGrowResult:
        return _kybra_ic._kybra_stable_grow(new_pages)  # type: ignore

    @staticmethod
    def stable_read(offset: nat32, length: nat32) -> blob:
        return _kybra_ic._kybra_stable_read(offset, length)  # type: ignore

    @staticmethod
    def stable_size() -> nat32:
        return _kybra_ic._kybra_stable_size()  # type: ignore

    @staticmethod
    def stable_write(offset: nat32, buf: blob):
        _kybra_ic._kybra_stable_write(offset, buf)  # type: ignore

    @staticmethod
    def stable64_grow(new_pages: nat64) -> Stable64GrowResult:
        return _kybra_ic._kybra_stable64_grow(new_pages)  # type: ignore

    @staticmethod
    def stable64_read(offset: nat64, length: nat64) -> blob:
        return _kybra_ic._kybra_stable64_read(offset, length)  # type: ignore

    @staticmethod
    def stable64_size() -> nat64:
        return _kybra_ic._kybra_stable64_size()  # type: ignore

    @staticmethod
    def stable64_write(offset: nat64, buf: blob):
        _kybra_ic._kybra_stable64_write(offset, buf)  # type: ignore

    @staticmethod
    def time() -> nat64:
        return _kybra_ic._kybra_time()  # type: ignore

    @staticmethod
    def trap(message: str) -> NoReturn:  # type: ignore
        _kybra_ic._kybra_trap(message)  # type: ignore


class Canister:
    canister_id: Principal

    def __init__(self, canister_id: Principal):
        self.canister_id = canister_id


P = ParamSpec("P")


class AsyncInfo:
    name: str
    args: list[Any]

    def __init__(self, name: str, args: list[Any]):
        self.name = name
        self.args = args

    def with_cycles(self, cycles: nat64) -> "AsyncInfo":
        return AsyncInfo("call_with_payment", [*self.args, cycles])

    def with_cycles128(self, cycles: nat) -> "AsyncInfo":
        return AsyncInfo("call_with_payment128", [*self.args, cycles])

    def notify(self) -> NotifyResult:
        qualname: str = self.args[1]
        with_payment = (
            "with_payment128_"
            if self.name == "call_with_payment" or self.name == "call_with_payment128"
            else ""
        )
        notify_function_name = (
            f'_kybra_notify_{with_payment}{qualname.replace(".", "_")}_wrapper'
        )

        return (
            getattr(_kybra_ic, notify_function_name)(self.args)  # type: ignore
        )


# TODO this decorator is removing the static type checking of the self parameter for instance methods
# TODO watch out for *kwargs
def service_method(func: Callable[P, T]) -> Callable[P, CanisterResult[T]]:
    def intermediate_func(*args):  # type: ignore
        the_self = args[0]  # type: ignore
        selfless_args = args[1:]  # type: ignore

        return AsyncInfo(
            "call",
            [
                the_self.canister_id,  # type: ignore
                func.__qualname__,
                *selfless_args
            ],
        )

    return intermediate_func  # type: ignore


def service_query(func: Callable[P, T]) -> Callable[P, CanisterResult[T]]:
    return service_method(func)


def service_update(func: Callable[P, T]) -> Callable[P, CanisterResult[T]]:
    return service_method(func)


K = TypeVar("K")
V = TypeVar("V")


class KeyTooLarge(Record):
    given: nat32
    max: nat32


class ValueTooLarge(Record):
    given: nat32
    max: nat32


class InsertError(Variant, total=False):
    KeyTooLarge: KeyTooLarge
    ValueTooLarge: ValueTooLarge


class InsertResult(Generic[V]):
    ok: V
    err: Optional[InsertError]

    def __init__(self, ok: V, err: InsertError):
        self.ok = ok
        self.err = err


class StableBTreeMap(Generic[K, V]):
    """
    A map based on a self-balancing tree that persists across canister upgrades.
    """

    def __init__(self, memory_id: nat8, max_key_size: int, max_value_size: int):
        """
        Initialize the stable B-tree map.

        :param memory_id: The memory ID of the stable B-tree map.
        :param max_key_size: The maximum size of the keys (in bytes) in the map.
        :param max_value_size: The maximum size of the values (in bytes) in the map.
        """
        self.memory_id = memory_id

    def contains_key(self, key: K) -> bool:
        """
        Check if the map contains a key.

        :param key: The key to check for in the map.
        :return: True if the key is in the map, False otherwise.
        """
        return _kybra_ic._kybra_stable_b_tree_map_contains_key(self.memory_id, key)  # type: ignore

    def get(self, key: K) -> opt[V]:
        """
        Get the value associated with a key in the map.

        :param key: The key to get the value for.
        :return: The value associated with the key, or None if the key is not in the map.
        """
        return _kybra_ic._kybra_stable_b_tree_map_get(self.memory_id, key)  # type: ignore

    def insert(self, key: K, value: V) -> InsertResult[opt[V]]:
        """
        Insert a key-value pair into the map.

        :param key: The key to insert.
        :param value: The value to insert.
        :return: An instance of InsertResult containing an ok attribute if the insertion succeeded
        or an err attribute if the insertion failed. If the insertion succeeded the ok attribute
        will contain the previous value associated with the key, if any. If the insertion failed,
        the err attribute will contain an instance of InsertError indicating the reason for the
        failure.
        """
        return _kybra_ic._kybra_stable_b_tree_map_insert(self.memory_id, key, value)  # type: ignore

    def is_empty(self) -> bool:
        """
        Check if the map is empty.

        :return: True if the map is empty, False otherwise.
        """
        return _kybra_ic._kybra_stable_b_tree_map_is_empty(self.memory_id)  # type: ignore

    def items(self) -> list[tuple[K, V]]:
        """
        Get a list of all key-value pairs in the map.

        :return: A list of tuples containing all key-value pairs in the map.
        """
        return _kybra_ic._kybra_stable_b_tree_map_items(self.memory_id)  # type: ignore

    def keys(self) -> list[K]:
        """
        Get a list of all keys in the map.

        :return: A list of all keys in the map.
        """
        return _kybra_ic._kybra_stable_b_tree_map_keys(self.memory_id)  # type: ignore

    def len(self) -> nat64:
        """
        Get the number of key-value pairs in the map.

        :return: The number of key-value pairs in the map.
        """
        return _kybra_ic._kybra_stable_b_tree_map_len(self.memory_id)  # type: ignore

    def remove(self, key: K) -> opt[V]:
        """
        Remove a key-value pair from the map.

        :param key: The key of the key-value pair to remove.
        :return: The value associated with the key, or None if the key is not in the map.
        """
        return _kybra_ic._kybra_stable_b_tree_map_remove(self.memory_id, key)  # type: ignore

    def values(self) -> list[V]:
        """
        Get a list of all values in the map.

        :return: A list of all values in the map.
        """
        return _kybra_ic._kybra_stable_b_tree_map_values(self.memory_id)  # type: ignore
