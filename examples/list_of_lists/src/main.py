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
    Vec,
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
def list_of_string_one(params: Vec[str]) -> Vec[str]:
    return params


@query
def list_of_string_two(params: Vec[Vec[str]]) -> Vec[Vec[str]]:
    return params


@query
def list_of_string_four(
    params: Vec[Vec[Vec[Vec[str]]]],
) -> Vec[Vec[Vec[Vec[str]]]]:
    return params


@query
def list_of_list_of_int8() -> Vec[Vec[Vec[Vec[Vec[Vec[Vec[int8]]]]]]]:
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
def list_of_null(param: Vec[Vec[Vec[null]]]) -> Vec[Vec[Vec[null]]]:
    return param


@query
def list_of_bool(param: Vec[Vec[Vec[bool]]]) -> Vec[Vec[Vec[bool]]]:
    return param


@query
def list_of_string(param: Vec[Vec[Vec[str]]]) -> Vec[Vec[Vec[str]]]:
    return param


@query
def list_of_option_string(
    param: Vec[Vec[Vec[opt[str]]]],
) -> Vec[Vec[Vec[opt[str]]]]:
    return param


@query
def list_of_empty() -> Vec[Vec[Vec[empty]]]:
    raise Exception("Anything you want")


@query
def list_of_reserved() -> Vec[Vec[Vec[reserved]]]:
    return [[["A"], ["n"]], [["y", "t", "h"], ["i", "n", "g"]]]


@query
def list_of_func(param: Vec[Vec[Vec[BasicFunc]]]) -> Vec[Vec[Vec[BasicFunc]]]:
    return param


@query
def list_of_principal(
    param: Vec[Vec[Vec[Principal]]],
) -> Vec[Vec[Vec[Principal]]]:
    return param


# TODO do I need to test Rejection Code


@query
def list_of_f64(param: Vec[Vec[Vec[float64]]]) -> Vec[Vec[Vec[float64]]]:
    return param


@query
def list_of_f32(param: Vec[Vec[Vec[float32]]]) -> Vec[Vec[Vec[float32]]]:
    return param


@query
def list_of_int(param: Vec[Vec[Vec[int]]]) -> Vec[Vec[Vec[int]]]:
    return param


@query
def list_of_int64(param: Vec[Vec[Vec[int64]]]) -> Vec[Vec[Vec[int64]]]:
    return param


@query
def list_of_int32(param: Vec[Vec[Vec[int32]]]) -> Vec[Vec[Vec[int32]]]:
    return param


@query
def list_of_int16(param: Vec[Vec[Vec[int16]]]) -> Vec[Vec[Vec[int16]]]:
    return param


@query
def list_of_int8(param: Vec[Vec[Vec[int8]]]) -> Vec[Vec[Vec[int8]]]:
    return param


@query
def list_of_nat(param: Vec[Vec[Vec[nat]]]) -> Vec[Vec[Vec[nat]]]:
    return param


@query
def list_of_nat64(param: Vec[Vec[Vec[nat64]]]) -> Vec[Vec[Vec[nat64]]]:
    return param


@query
def list_of_nat32(param: Vec[Vec[Vec[nat32]]]) -> Vec[Vec[Vec[nat32]]]:
    return param


@query
def list_of_nat16(param: Vec[Vec[Vec[nat16]]]) -> Vec[Vec[Vec[nat16]]]:
    return param


@query
def list_of_nat8(param: Vec[Vec[Vec[nat8]]]) -> Vec[Vec[Vec[nat8]]]:
    return param


@query
def list_of_record(param: Vec[Vec[Vec[Person]]]) -> Vec[Vec[Vec[Person]]]:
    return param


@query
def list_of_variant(param: Vec[Vec[Vec[State]]]) -> Vec[Vec[Vec[State]]]:
    return param


@query
def list_of_blob(param: Vec[blob]) -> Vec[blob]:
    return param


@query
def list_of_list_of_blob(param: Vec[Vec[blob]]) -> Vec[Vec[blob]]:
    return param
