# TODO Add in Canister types like in the Azle tuple_types example

from kybra import nat64, null, Principal, query, Record, Variant


class User(Record):
    id: str
    primitive_two_tuple: "PrimitiveTwoTuple"


class Reaction(Variant, total=False):
    Good: null
    Bad: "ComplexThreeTuple"


PrimitiveOneTuple = tuple[str]
PrimitiveTwoTuple = tuple[str, nat64]
PrimitiveThreeTuple = tuple[str, nat64, Principal]

ComplexOneTuple = tuple[PrimitiveTwoTuple]
ComplexTwoTuple = tuple[PrimitiveTwoTuple, User]
ComplexThreeTuple = tuple[PrimitiveTwoTuple, User, Reaction]

Header = tuple[str, str]


class HttpResponse(Record):
    headers: list[Header]


class StreamingCallbackType(Variant, total=False):
    with_headers: list[Header]
    without_headers: null


@query
def primitive_one_tuple_return_type() -> PrimitiveOneTuple:
    return ('Hello',)


@query
def primitive_one_tuple_param(param: PrimitiveOneTuple) -> PrimitiveOneTuple:
    return param


@query
def primitive_one_tuple_inline_return_type() -> tuple[str]:
    return ('Greenland',)


@query
def primitive_one_tuple_inline_param(param: tuple[str]) -> tuple[str]:
    return param


@query
def primitive_two_tuple_return_type() -> PrimitiveTwoTuple:
    return ('Content-Type', 64)


@query
def primitive_two_tuple_param(param: PrimitiveTwoTuple) -> PrimitiveTwoTuple:
    return param


@query
def primitive_two_tuple_inline_return_type() -> tuple[str, str]:
    return ('Fun', 'Times')


@query
def primitive_two_tuple_inline_param(param: tuple[str, str]) -> tuple[str, str]:
    return param


@query
def primitive_three_tuple_return_type() -> PrimitiveThreeTuple:
    return ('Good', 454, Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai'))


@query
def primitive_three_tuple_param(param: PrimitiveThreeTuple) -> PrimitiveThreeTuple:
    return param


@query
def primitive_three_tuple_inline_return_type() -> tuple[str, nat64, Principal]:
    return ('Fun', 101, Principal.from_str('aaaaa-aa'))


@query
def primitive_three_tuple_inline_param(param: tuple[str, nat64, Principal]) -> tuple[str, nat64, Principal]:
    return param


@query
def complex_one_tuple_return_type() -> ComplexOneTuple:
    return (('Hello', 0),)


@query
def complex_one_tuple_param(param: ComplexOneTuple) -> ComplexOneTuple:
    return param


@query
def complex_one_tuple_inline_return_type() -> tuple[PrimitiveTwoTuple]:
    return (('Candy', 56),)


@query
def complex_one_tuple_inline_param(param: tuple[PrimitiveTwoTuple]) -> tuple[PrimitiveTwoTuple]:
    return param


@query
def complex_two_tuple_return_type() -> ComplexTwoTuple:
    return (
        ('Content-Type', 64),
        {
            'id': '0',
            'primitive_two_tuple': ('Content-Type', 64)
        }
    )


@query
def complex_two_tuple_param(param: ComplexTwoTuple) -> ComplexTwoTuple:
    return param


@query
def complex_two_tuple_inline_return_type() -> tuple[PrimitiveTwoTuple, User]:
    return (
        ('Content-Type', 644),
        {
            'id': '444',
            'primitive_two_tuple': ('Content-Type', 6_422)
        }
    )


@query
def complex_two_tuple_inline_param(param: tuple[PrimitiveTwoTuple, User]) -> tuple[PrimitiveTwoTuple, User]:
    return param


@query
def complex_three_tuple_return_type() -> ComplexThreeTuple:
    return (
        ('Content-Type', 64),
        {
            'id': '0',
            'primitive_two_tuple': ('Content-Type', 64)
        },
        {
            'Bad': (
                ('Content-Type', 64),
                {
                    'id': '1',
                    'primitive_two_tuple': ('Content-Type', 64)
                },
                {
                    'Good': None
                }
            )
        }
    )


@query
def complex_three_tuple_param(param: ComplexThreeTuple) -> ComplexThreeTuple:
    return param


@query
def complex_three_tuple_inline_return_type() -> tuple[PrimitiveTwoTuple, User, Reaction]:
    return (
        ('Content-Type', 64),
        {
            'id': '0',
            'primitive_two_tuple': ('Content-Type', 64)
        },
        {
            'Bad': (
                ('Content-Type', 64),
                {
                    'id': '1',
                    'primitive_two_tuple': ('Content-Type', 64)
                },
                {
                    'Good': None
                }
            )
        }
    )


@query
def complex_three_tuple_inline_param(param: tuple[PrimitiveTwoTuple, User, Reaction]) -> tuple[PrimitiveTwoTuple, User, Reaction]:
    return param


@query
def tuple_array_params_and_return_type(headers: list[Header]) -> list[Header]:
    return headers


@query
def tuple_array_record_field() -> HttpResponse:
    return {
        'headers': [
            ('Content-Type', 'application/json'),
            ('Accept-Ranges', 'bytes')
        ]
    }


@query
def tuple_array_variant_field() -> StreamingCallbackType:
    return {
        'with_headers': [
            ('Content-Type', 'application/json'),
            ('Accept-Ranges', 'bytes')
        ]
    }
