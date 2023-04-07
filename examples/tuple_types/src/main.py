# TODO Add in Canister types like in the Azle tuple_types example

from kybra import nat8, nat64, null, Principal, query, Record, Tuple, Vec, Variant


class User(Record):
    id: str
    primitive_two_tuple: "PrimitiveTwoTuple"


class Reaction(Variant, total=False):
    Good: null
    Bad: "ComplexThreeTuple"


PrimitiveOneTuple = Tuple[str]
PrimitiveTwoTuple = Tuple[str, nat64]
PrimitiveThreeTuple = Tuple[str, nat64, Principal]

ComplexOneTuple = Tuple[PrimitiveTwoTuple]
ComplexTwoTuple = Tuple[PrimitiveTwoTuple, User]
ComplexThreeTuple = Tuple[PrimitiveTwoTuple, User, Reaction]

Header = Tuple[str, str]


class HttpResponse(Record):
    headers: Vec[Header]


class StreamingCallbackType(Variant, total=False):
    WithHeaders: Vec[Header]
    WithoutHeaders: null


@query
def primitive_one_tuple_return_type() -> PrimitiveOneTuple:
    return ("Hello",)


@query
def primitive_one_tuple_param(param: PrimitiveOneTuple) -> PrimitiveOneTuple:
    return param


@query
def primitive_one_tuple_inline_return_type() -> Tuple[str]:
    return ("Greenland",)


@query
def primitive_one_tuple_inline_param(param: Tuple[str]) -> Tuple[str]:
    return param


@query
def primitive_two_tuple_return_type() -> PrimitiveTwoTuple:
    return ("Content-Type", 64)


@query
def primitive_two_tuple_param(param: PrimitiveTwoTuple) -> PrimitiveTwoTuple:
    return param


@query
def primitive_two_tuple_inline_return_type() -> Tuple[str, str]:
    return ("Fun", "Times")


@query
def primitive_two_tuple_inline_param(param: Tuple[str, str]) -> Tuple[str, str]:
    return param


@query
def primitive_three_tuple_return_type() -> PrimitiveThreeTuple:
    return ("Good", 454, Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai"))


@query
def primitive_three_tuple_param(param: PrimitiveThreeTuple) -> PrimitiveThreeTuple:
    return param


@query
def primitive_three_tuple_inline_return_type() -> Tuple[str, nat64, Principal]:
    return ("Fun", 101, Principal.from_str("aaaaa-aa"))


@query
def primitive_three_tuple_inline_param(
    param: Tuple[str, nat64, Principal]
) -> Tuple[str, nat64, Principal]:
    return param


@query
def complex_one_tuple_return_type() -> ComplexOneTuple:
    return (("Hello", 0),)


@query
def complex_one_tuple_param(param: ComplexOneTuple) -> ComplexOneTuple:
    return param


@query
def complex_one_tuple_inline_return_type() -> Tuple[PrimitiveTwoTuple]:
    return (("Candy", 56),)


@query
def complex_one_tuple_inline_param(
    param: Tuple[PrimitiveTwoTuple],
) -> Tuple[PrimitiveTwoTuple]:
    return param


@query
def complex_two_tuple_return_type() -> ComplexTwoTuple:
    return (
        ("Content-Type", 64),
        {"id": "0", "primitive_two_tuple": ("Content-Type", 64)},
    )


@query
def complex_two_tuple_param(param: ComplexTwoTuple) -> ComplexTwoTuple:
    return param


@query
def complex_two_tuple_inline_return_type() -> Tuple[PrimitiveTwoTuple, User]:
    return (
        ("Content-Type", 644),
        {"id": "444", "primitive_two_tuple": ("Content-Type", 6_422)},
    )


@query
def complex_two_tuple_inline_param(
    param: Tuple[PrimitiveTwoTuple, User]
) -> Tuple[PrimitiveTwoTuple, User]:
    return param


@query
def complex_three_tuple_return_type() -> ComplexThreeTuple:
    return (
        ("Content-Type", 64),
        {"id": "0", "primitive_two_tuple": ("Content-Type", 64)},
        {
            "Bad": (
                ("Content-Type", 64),
                {"id": "1", "primitive_two_tuple": ("Content-Type", 64)},
                {"Good": None},
            )
        },
    )


@query
def complex_three_tuple_param(param: ComplexThreeTuple) -> ComplexThreeTuple:
    return param


@query
def complex_three_tuple_inline_return_type() -> (
    Tuple[PrimitiveTwoTuple, User, Reaction]
):
    return (
        ("Content-Type", 64),
        {"id": "0", "primitive_two_tuple": ("Content-Type", 64)},
        {
            "Bad": (
                ("Content-Type", 64),
                {"id": "1", "primitive_two_tuple": ("Content-Type", 64)},
                {"Good": None},
            )
        },
    )


@query
def complex_three_tuple_inline_param(
    param: Tuple[PrimitiveTwoTuple, User, Reaction]
) -> Tuple[PrimitiveTwoTuple, User, Reaction]:
    return param


@query
def tuple_array_params_and_return_type(headers: Vec[Header]) -> Vec[Header]:
    return headers


@query
def tuple_array_record_field() -> HttpResponse:
    return {
        "headers": [("Content-Type", "application/json"), ("Accept-Ranges", "bytes")]
    }


@query
def tuple_array_variant_field() -> StreamingCallbackType:
    return {
        "WithHeaders": [
            ("Content-Type", "application/json"),
            ("Accept-Ranges", "bytes"),
        ]
    }


@query
def nested_tuple_query(
    param: Tuple[Tuple[str, Tuple[nat8, nat8]], int]
) -> Tuple[Tuple[str, Tuple[nat8, nat8]], int]:
    return param
