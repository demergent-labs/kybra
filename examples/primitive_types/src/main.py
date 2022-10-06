from kybra import (
    query,
    int64,
    int32,
    int16,
    int8,
    nat,
    nat64,
    nat32,
    nat16,
    nat8,
    float64,
    float32,
    text
)
import math

@query
def get_int() -> int:
    return 170_141_183_460_469_231_731_687_303_715_884_105_727

@query
def get_int64() -> int64:
    return 9_223_372_036_854_775_807

@query
def get_int32() -> int32:
    return 2_147_483_647

@query
def get_int16() -> int16:
    return 32_767

@query
def get_int8() -> int8:
    return 127

@query
def get_nat() -> nat:
    return 340_282_366_920_938_463_463_374_607_431_768_211_455

@query
def get_nat64() -> nat64:
    return 18_446_744_073_709_551_615

@query
def get_nat32() -> nat32:
    return 4_294_967_295

@query
def get_nat16() -> nat16:
    return 65_535

@query
def get_nat8() -> nat8:
    return 255

@query
def get_float64() -> float64:
    return math.e

@query
def get_float32() -> float32:
    return math.pi

@query
def get_text() -> text:
    return 'this is a string defined with text'

@query
def get_string() -> str:
    return 'this is a string defined with str'

@query
def get_bool() -> bool:
    return True
