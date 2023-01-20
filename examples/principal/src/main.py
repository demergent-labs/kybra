from kybra import blob, null, Principal, query, Record, Variant


class User(Record):
    id: Principal
    username: str


class Status(Variant, total=False):
    WaitingOn: Principal
    Online: null
    Offline: null


@query
def principal_return_type() -> Principal:
    return Principal.from_str('aaaaa-aa')


@query
def principal_param(principal: Principal) -> Principal:
    return principal


@query
def principal_in_record() -> User:
    return {
        'id': Principal.from_str('aaaaa-aa'),
        'username': 'lastmjs'
    }


@query
def principal_in_variant() -> Status:
    return {
        'WaitingOn': Principal.from_str('aaaaa-aa')
    }


@query
def principal_from_hex(principal_hex: str) -> Principal:
    return Principal.from_hex(principal_hex)


@query
def principal_from_text(principal_text: str) -> Principal:
    return Principal.from_str(principal_text)


@query
def principal_from_blob(principal_bytes: blob) -> Principal:
    return Principal(principal_bytes)


@query
def principal_to_hex(principal: Principal) -> str:
    return principal.hex


@query
def principal_to_text(principal: Principal) -> str:
    return principal.to_str()


@query
def principal_to_blob(principal: Principal) -> blob:
    return principal.bytes


@query
def principal_self_authenticating(public_key: blob) -> Principal:
    return Principal.self_authenticating(public_key)
