from kybra import empty, ic, Manual, query


@query
def accept() -> bool:
    return True


@query
def error() -> Manual[empty]:
    ...  # This errors because neither ic.reject nor ic.reply were called


@query
def reject(message: str) -> Manual[empty]:
    ic.reject(message)
