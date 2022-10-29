from kybra import blob, ic, query

@query
def candid_encode(candid_string: str) -> blob:
    return ic.candid_encode(candid_string)

@query
def candid_decode(candid_encoded: blob) -> str:
    return ic.candid_decode(candid_encoded)
