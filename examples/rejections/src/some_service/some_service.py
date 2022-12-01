from kybra import empty, ic, manual, query

@query
def accept() -> bool:
    return True

@query
def error() -> manual[empty]: ... # This errors because neither ic.reject nor ic.reply were called

@query
def reject(message: str) -> manual[empty]:
    ic.reject(message)
