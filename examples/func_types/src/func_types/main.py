from typing import TypeAlias
from kybra import (
    Async,
    blob,
    Func,
    Canister,
    CanisterResult,
    canisters,
    method,
    nat64,
    Oneway,
    Principal,
    Query,
    query,
    Record,
    Update,
    update,
    Variant,
)


class User(Record):
    id: str
    basic_func: "BasicFunc"
    complex_func: "ComplexFunc"


class Reaction(Variant, total=False):  # TODO Make sure I got all variants
    Good: None
    Bad: None
    BasicFunc: "BasicFunc"
    ComplexFunc: "ComplexFunc"


# class GetNotifierFromNotifiersCanisterResult(Variant, total=False):
#     ok: "NotifierFunc"
#     err: str


BasicFunc: TypeAlias = Func(Query[[str], str])
ComplexFunc: TypeAlias = Func(Update[[User, Reaction], nat64])
# NotifierFunc: TypeAlias = Func(Oneway[[blob], None])


# class Canister2(Canister):
#     @method
#     def get_notifier() -> NotifierFunc:
#         ...


@query
def basic_func_param(basic_func: BasicFunc) -> BasicFunc:
    return basic_func


@query
def basic_func_param_array(basic_funcs: list[BasicFunc]) -> list[BasicFunc]:
    return basic_funcs


@query
def basic_func_return_type() -> BasicFunc:
    return (Principal.from_str("aaaaa-aa"), "create_canister")


@query
def basic_func_return_type_array() -> list[BasicFunc]:
    return [
        (Principal.from_str("aaaaa-aa"), "create_canister"),
        (Principal.from_str("aaaaa-aa"), "update_setttings"),
        (Principal.from_str("aaaaa-aa"), "install_code"),
    ]


@query
def complex_func_param(complex_func: ComplexFunc) -> ComplexFunc:
    return complex_func


@query
def complex_func_return_type() -> ComplexFunc:
    return (Principal.from_str("aaaaa-aa"), "stop_canister")


# @update
# def get_notifier_from_notifiers_canister() -> Async[
#     GetNotifierFromNotifiersCanisterResult
# ]:
#     notifiers_canister = Canister2(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))

#     result: CanisterResult[NotifierFunc] = yield notifiers_canister.get_notifier()

#     if result.err is not None:
#         return {"err": result.err}

#     return {"ok": result.ok}
