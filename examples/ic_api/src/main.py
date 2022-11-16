from kybra import ic, Principal, query

# returns the principal of the identity that called this function
@query
def caller() -> Principal:
    return ic.caller()
