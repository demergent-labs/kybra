from kybra import (
    blob,
    empty,
    float32,
    float64,
    Func,
    int8,
    int16,
    int32,
    int64,
    nat,
    nat8,
    nat16,
    nat32,
    nat64,
    null,
    opt,
    Principal,
    Query,
    query,
    Record,
    reserved,
    Variant,
)


class Person(Record):
    name: str
    age: nat8


class State(Variant, total=False):
    solid: null
    liquid: null
    gas: null


BasicFunc = Func(Query[[str], str])


@query
def list_of_string_one(params: list[str]) -> list[str]:
    return params


@query
def list_of_string_two(params: list[list[str]]) -> list[list[str]]:
    return params


@query
def list_of_string_four(
    params: list[list[list[list[str]]]],
) -> list[list[list[list[str]]]]:
    return params


@query
def list_of_list_of_int8() -> list[list[list[list[list[list[list[int8]]]]]]]:
    return [
        [
            [
                [[[[1], [2]], [[1, 2, 3], [4, 5, 6]]]],
                [[[[1]]], [[[2]]]],
                [[[[3]]]],
            ]
        ],
        [
            [
                [[[[1]]]],
                [[[[2]]]],
            ],
            [
                [[[[3]]]],
                [[[[4]]]],
            ],
        ],
    ]


@query
def list_of_null(param: list[list[list[null]]]) -> list[list[list[null]]]:
    return param


@query
def list_of_bool(param: list[list[list[bool]]]) -> list[list[list[bool]]]:
    return param


@query
def list_of_string(param: list[list[list[str]]]) -> list[list[list[str]]]:
    return param


@query
def list_of_option_string(
    param: list[list[list[opt[str]]]],
) -> list[list[list[opt[str]]]]:
    return param


@query
def list_of_empty() -> list[list[list[empty]]]:
    raise Exception("Anything you want")


@query
def list_of_reserved() -> list[list[list[reserved]]]:
    return [[["A"], ["n"]], [["y", "t", "h"], ["i", "n", "g"]]]


@query
def list_of_func(param: list[list[list[BasicFunc]]]) -> list[list[list[BasicFunc]]]:
    return param


@query
def list_of_principal(
    param: list[list[list[Principal]]],
) -> list[list[list[Principal]]]:
    return param


# TODO do I need to test Rejection Code


@query
def list_of_f64(param: list[list[list[float64]]]) -> list[list[list[float64]]]:
    return param


@query
def list_of_f32(param: list[list[list[float32]]]) -> list[list[list[float32]]]:
    return param


@query
def list_of_int(param: list[list[list[int]]]) -> list[list[list[int]]]:
    return param


@query
def list_of_int64(param: list[list[list[int64]]]) -> list[list[list[int64]]]:
    return param


@query
def list_of_int32(param: list[list[list[int32]]]) -> list[list[list[int32]]]:
    return param


@query
def list_of_int16(param: list[list[list[int16]]]) -> list[list[list[int16]]]:
    return param


@query
def list_of_int8(param: list[list[list[int8]]]) -> list[list[list[int8]]]:
    return param


@query
def list_of_nat(param: list[list[list[nat]]]) -> list[list[list[nat]]]:
    return param


@query
def list_of_nat64(param: list[list[list[nat64]]]) -> list[list[list[nat64]]]:
    return param


@query
def list_of_nat32(param: list[list[list[nat32]]]) -> list[list[list[nat32]]]:
    return param


@query
def list_of_nat16(param: list[list[list[nat16]]]) -> list[list[list[nat16]]]:
    return param


@query
def list_of_nat8(param: list[list[list[nat8]]]) -> list[list[list[nat8]]]:
    return param


@query
def list_of_record(param: list[list[list[Person]]]) -> list[list[list[Person]]]:
    return param


@query
def list_of_variant(param: list[list[list[State]]]) -> list[list[list[State]]]:
    return param


@query
def list_of_blob(param: list[blob]) -> list[blob]:
    return param


@query
def list_of_list_of_blob(param: list[list[blob]]) -> list[list[blob]]:
    return param
