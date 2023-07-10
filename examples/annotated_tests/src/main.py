from typing import TypeAlias
from kybra import Alias, Func, FuncTuple, nat8, Principal, query, StableBTreeMap, Query


stable_map0: StableBTreeMap[nat8, str] = StableBTreeMap[nat8, str](
    memory_id=3, max_key_size=100, max_value_size=100
)


BoolAlias: TypeAlias = Alias[bool]


BasicFunc: type[FuncTuple] = Func(Query[[str], str])


@query
def is_empty() -> bool:
    return stable_map0.is_empty()


@query
def get_type_alias() -> BoolAlias:
    return True


@query
def get_func() -> BasicFunc:
    return (Principal.from_str("aaaaa-aa"), "create_canister")
