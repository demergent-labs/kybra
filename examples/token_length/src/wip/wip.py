# type: ignore
# This file has a bunch of expressions that do not get the token length
# calculated correctly and therefore would not be properly highlighted in
# errors. This file is a sub set of the breaks.py and is here to help focus on a
# handful of tokens

BasicFunc: TypeAlias = Func(Query[[str], str])

Query[[str], str]

BasicFunc: TypeAlias = Func(Query[[str], str])
ComplexFunc: TypeAlias = Func(Update[[User, Reaction], nat64])


def my_function_4(arg1, *args, keyword_arg=None):
    if False:
        pass
    elif True:
        pass
    print("arg1:", arg1)
    print("args:", args)
    print("keyword_arg:", keyword_arg)
    print("kwargs:", kwargs)


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
    ok: "NotifierFunc"
    err: str


BasicFunc: TypeAlias = Func(Query[[str], str])
ComplexFunc: TypeAlias = Func(Update[[User, Reaction], nat64])
StableFunc: TypeAlias = Func(Query[[nat64, str], void])
NullFunc: TypeAlias = Func(
    Query[[opt[null], list[null], null, list[list[null]], list[opt[null]]], null]
)


stable_storage = StableBTreeMap[str, StableFunc](
    memory_id=0, max_key_size=25, max_value_size=1_000
)


@init
def init_():
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
def basic_func_param_array(basic_funcs: list[BasicFunc]) -> list[BasicFunc]:
    return basic_funcs


@query
def basic_func_return_type() -> BasicFunc:
    return (Principal.from_str("aaaaa-aa"), "create_canister")


@query
def basic_func_return_type_array() -> list[BasicFunc]:
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

    result: CanisterResult[NotifierFunc] = yield notifiers_canister.get_notifier()

    if result.err is not None:
        return {"err": result.err}

    return {"ok": result.ok}
