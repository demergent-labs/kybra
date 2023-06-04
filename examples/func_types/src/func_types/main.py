from kybra import (
    Async,
    Func,
    CallResult,
    init,
    match,
    nat64,
    null,
    Opt,
    Principal,
    Query,
    query,
    Record,
    StableBTreeMap,
    Update,
    update,
    Variant,
    Vec,
    void,
)

from src.notifiers.types import Notifier, NotifierFunc


class User(Record):
    id: str
    basic_func: "BasicFunc"
    complex_func: "ComplexFunc"


class Reaction(Variant, total=False):
    Good: null
    Bad: null
    BasicFunc: "BasicFunc"
    ComplexFunc: "ComplexFunc"


class GetNotifierFromNotifiersCanisterResult(Variant, total=False):
    Ok: "NotifierFunc"
    Err: str


BasicFunc = Func(Query[[str], str])
ComplexFunc = Func(Update[[User, Reaction], nat64])
StableFunc = Func(Query[[nat64, str], void])
NullFunc = Func(
    Query[[Opt[null], Vec[null], null, Vec[Vec[null]], Vec[Opt[null]]], null]
)


stable_storage = StableBTreeMap[str, StableFunc](
    memory_id=3, max_key_size=25, max_value_size=1_000
)


@init
def init_() -> void:
    stable_storage.insert(
        "stable_func", (Principal.from_str("aaaaa-aa"), "start_canister")
    )


@query
def get_stable_func() -> StableFunc:
    result = stable_storage.get("stable_func")
    if result:
        return result
    return (Principal.from_str("aaaaa-aa"), "raw_rand")


@query
def null_func_param(null_func: NullFunc) -> NullFunc:
    return null_func


@query
def basic_func_param(basic_func: BasicFunc) -> BasicFunc:
    return basic_func


@query
def basic_func_param_array(basic_funcs: Vec[BasicFunc]) -> Vec[BasicFunc]:
    return basic_funcs


@query
def basic_func_return_type() -> BasicFunc:
    return (Principal.from_str("aaaaa-aa"), "create_canister")


@query
def basic_func_return_type_array() -> Vec[BasicFunc]:
    return [
        (Principal.from_str("aaaaa-aa"), "create_canister"),
        (Principal.from_str("aaaaa-aa"), "update_settings"),
        (Principal.from_str("aaaaa-aa"), "install_code"),
    ]


@query
def complex_func_param(complex_func: ComplexFunc) -> ComplexFunc:
    return complex_func


@query
def complex_func_return_type() -> ComplexFunc:
    return (Principal.from_str("aaaaa-aa"), "stop_canister")


@update
def get_notifier_from_notifiers_canister() -> (
    Async[GetNotifierFromNotifiersCanisterResult]
):
    notifiers_canister = Notifier(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))

    result: CallResult[NotifierFunc] = yield notifiers_canister.get_notifier()

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )
