from kybra import alias, query, Result

SimpleResult = alias[Result[str, int]]


@query
def simple_result() -> SimpleResult:
    return SimpleResult("", 0)


NonGenericResultAlias = alias[Result[str, bool]]


@query
def nonGenericResultAlias() -> NonGenericResultAlias:
    return NonGenericResultAlias("", False)
