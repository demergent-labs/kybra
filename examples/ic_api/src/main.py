from kybra import ic, nat64, Principal, query

# returns the principal of the identity that called this function
@query
def caller() -> Principal:
    return ic.caller()

@query
def performance_counter() -> nat64:
    return ic.performance_counter(0)
