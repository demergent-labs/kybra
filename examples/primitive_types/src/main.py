from kybra import (
    empty,
    float64,
    float32,
    ic,
    int64,
    int32,
    int16,
    int8,
    nat,
    nat64,
    nat32,
    nat16,
    nat8,
    null,
    Principal,
    reserved,
    query,
    text
)
import math


@query
def get_string() -> str:
    return 'string'


@query
def print_string(str: str) -> str:
    ic.print(type(str))
    return str


@query
def get_text() -> text:
    return 'text'


@query
def print_text(text: text) -> text:
    ic.print(type(text))
    return text


@query
def get_int() -> int:
    return 170_141_183_460_469_231_731_687_303_715_884_105_727


@query
def print_int(int: int) -> int:
    ic.print(type(int))
    return int


@query
def get_int64() -> int64:
    return 9_223_372_036_854_775_807


@query
def print_int64(int64: int64) -> int64:
    ic.print(type(int64))
    return int64


@query
def get_int32() -> int32:
    return 2_147_483_647


@query
def print_int32(int32: int32) -> int32:
    ic.print(type(int32))
    return int32


@query
def get_int16() -> int16:
    return 32_767


@query
def print_int16(int16: int16) -> int16:
    ic.print(type(int16))
    return int16


@query
def get_int8() -> int8:
    return 127


@query
def print_int8(int8: int8) -> int8:
    ic.print(type(int8))
    return int8


@query
def get_nat() -> nat:
    return 340_282_366_920_938_463_463_374_607_431_768_211_455


@query
def print_nat(nat: nat) -> nat:
    ic.print(type(nat))
    return nat


@query
def get_nat64() -> nat64:
    return 18_446_744_073_709_551_615


@query
def print_nat64(nat64: nat64) -> nat64:
    ic.print(type(nat64))
    return nat64


@query
def get_nat32() -> nat32:
    return 4_294_967_295


@query
def print_nat32(nat32: nat32) -> nat32:
    ic.print(type(nat32))
    return nat32


@query
def get_nat16() -> nat16:
    return 65_535


@query
def print_nat16(nat16: nat16) -> nat16:
    ic.print(type(nat16))
    return nat16


@query
def get_nat8() -> nat8:
    return 255


@query
def print_nat8(nat8: nat8) -> nat8:
    ic.print(type(nat8))
    return nat8


@query
def get_float64() -> float64:
    return math.e


@query
def print_float64(float64: float64) -> float64:
    ic.print(type(float64))
    return float64


@query
def get_float32() -> float32:
    return math.pi


@query
def print_float32(float32: float32) -> float32:
    ic.print(type(float32))
    return float32


@query
def get_bool() -> bool:
    return True


@query
def print_bool(bool: bool) -> bool:
    ic.print(type(bool))
    return bool


@query
def get_principal() -> Principal:
    return Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai')


@query
def print_principal(principal: Principal) -> Principal:
    ic.print(type(principal))
    return principal


@query
def get_null() -> null:
    return None


@query
def print_null(null: null) -> null:
    ic.print(type(null))
    return null


@query
def get_reserved() -> reserved:
    return 'anything'


@query
def print_reserved(reserved: reserved) -> reserved:
    ic.print(type(reserved))
    return reserved


@query
def get_empty() -> empty:
    raise Exception('Anything you want')


# Note: It is impossible to call this function because it requires an argument
# but there is no way to pass an "empty" value as an argument.
@query
def print_empty(empty: empty) -> empty:
    ic.print(type(empty))
    raise Exception('Anything you want')
