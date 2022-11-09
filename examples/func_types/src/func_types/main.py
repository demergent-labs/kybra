from typing import TypeAlias
from kybra import Func, query, Query

BasicFunc: TypeAlias = Func(Query[[str], str])


@query
def basic_func_param(basic_func: BasicFunc) -> BasicFunc:
    return basic_func
