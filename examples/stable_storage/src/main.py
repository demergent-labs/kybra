from kybra import blob, Func, ic, init, null, opt, post_upgrade, pre_upgrade, Principal, query, Query, Record, update, Variant
from kybra import float64, float32
from kybra import int64, int32, int16, int8
from kybra import nat, nat64, nat32, nat16, nat8
from typing import TypeAlias, TypedDict
import math

class User(Record):
    id: str
    children: list['Child']

class Child(Record):
    id: str

class Reaction(Variant, total=False):
    Fire: None
    Great: None

StableFunc: TypeAlias = Func(Query[[str], bool])

class StableStorage(TypedDict):
    stable_blob: blob
    stable_blobs: list[blob]
    stable_int: int
    stable_ints: list[int]
    stable_int64: int64
    stable_int32: int32
    stable_int16: int16
    stable_int8: int8
    stable_nat: nat
    stable_nat64: nat64
    stable_nat32: nat32
    stable_nat16: nat16
    stable_nat8: nat8
    stable_float64: float64
    stable_float32: float32
    stable_string: str
    stable_principal: Principal
    stable_user: User
    stable_users: list[User]
    stable_reaction: Reaction
    stable_func: StableFunc
    stable_boolean: bool
    stable_null: None
    stable_opt: opt[int]

stable_storage: StableStorage = ic.stable_storage()

@init
def init_():
    ic.print('init')

    stable_storage['stable_blob'] = bytes([0, 1, 2, 3, 4, 5])
    stable_storage['stable_blobs'] = [
        bytes([0, 1, 2, 3, 4, 5]),
        bytes([0, 1, 2, 3, 4, 5])
    ]
    stable_storage['stable_int'] = 170_141_183_460_469_231_731_687_303_715_884_105_727
    stable_storage['stable_ints'] = [
        170_141_183_460_469_231_731_687_303_715_884_105_727,
        170_141_183_460_469_231_731_687_303_715_884_105_727
    ]
    stable_storage['stable_int64'] = 9_223_372_036_854_775_807
    stable_storage['stable_int32'] = 2_147_483_647
    stable_storage['stable_int16'] = 32_767
    stable_storage['stable_int8'] = 127
    stable_storage['stable_nat'] = 340_282_366_920_938_463_463_374_607_431_768_211_455
    stable_storage['stable_nat64'] = 18_446_744_073_709_551_615
    stable_storage['stable_nat32'] = 4_294_967_295
    stable_storage['stable_nat16'] = 65_535
    stable_storage['stable_nat8'] = 255
    stable_storage['stable_float64'] = math.e
    stable_storage['stable_float32'] = math.pi
    stable_storage['stable_string'] = 'Hello there';
    stable_storage['stable_principal'] = Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai')
    stable_storage['stable_user'] = {
        'id': '0',
        'children': [
            {
                'id': '1'
            }
        ]
    }
    stable_storage['stable_reaction'] = {
        'Fire': None
    }
    stable_storage['stable_func'] = (Principal.from_str('aaaaa-aa'), 'raw_rand')
    stable_storage['stable_boolean'] = True
    stable_storage['stable_null'] = None
    stable_storage['stable_opt'] = None

@pre_upgrade
def pre_upgrade_():
    ic.print('pre_upgrade')

@post_upgrade
def post_upgrade_():
    ic.print('post_upgrade')

@query
def read_stable_blob() -> blob:
    return stable_storage['stable_blob']

@update
def write_stable_blob(blob: blob):
    stable_storage['stable_blob'] = blob

@query
def read_stable_blobs() -> list[blob]:
    return stable_storage['stable_blobs']

@update
def write_stable_blobs(blobs: list[blob]):
    stable_storage['stable_blobs'] = blobs

@query
def read_stable_int() -> int:
    return stable_storage['stable_int']

@update
def write_stable_int(int: int):
    stable_storage['stable_int'] = int

@query
def read_stable_ints() -> list[int]:
    return stable_storage['stable_ints']

@update
def write_stable_ints(ints: list[int]):
    stable_storage['stable_ints'] = ints

@query
def read_stable_int64() -> int64:
    return stable_storage['stable_int64']

@update
def write_stable_int64(int64: int64):
    stable_storage['stable_int64'] = int64

@query
def read_stable_int32() -> int32:
    return stable_storage['stable_int32']

@update
def write_stable_int32(int32: int32):
    stable_storage['stable_int32'] = int32

@query
def read_stable_int16() -> int16:
    return stable_storage['stable_int16']

@update
def write_stable_int16(int16: int16):
    stable_storage['stable_int16'] = int16

@query
def read_stable_int8() -> int8:
    return stable_storage['stable_int8']

@update
def write_stable_int8(int8: int8):
    stable_storage['stable_int8'] = int8

@query
def read_stable_nat() -> nat:
    return stable_storage['stable_nat']

@update
def write_stable_nat(nat: nat):
    stable_storage['stable_nat'] = nat

@query
def read_stable_nat64() -> nat64:
    return stable_storage['stable_nat64']

@update
def write_stable_nat64(nat64: nat64):
    stable_storage['stable_nat64'] = nat64

@query
def read_stable_nat32() -> nat32:
    return stable_storage['stable_nat32']

@update
def write_stable_nat32(nat32: nat32):
    stable_storage['stable_nat32'] = nat32

@query
def read_stable_nat16() -> nat16:
    return stable_storage['stable_nat16']

@update
def write_stable_nat16(nat16: nat16):
    stable_storage['stable_nat16'] = nat16

@query
def read_stable_nat8() -> nat8:
    return stable_storage['stable_nat8']

@update
def write_stable_nat8(nat8: nat8):
    stable_storage['stable_nat8'] = nat8

@query
def read_stable_float64() -> float64:
    return stable_storage['stable_float64']

@update
def write_stable_float64(float64: float64):
    stable_storage['stable_float64'] = float64

@query
def read_stable_float32() -> float32:
    return stable_storage['stable_float32']

@update
def write_stable_float32(float32: float32):
    stable_storage['stable_float32'] = float32

@query
def read_stable_string() -> str:
    return stable_storage['stable_string']

@update
def write_stable_string(string: str):
    stable_storage['stable_string'] = string

@query
def read_stable_principal() -> Principal:
    return stable_storage['stable_principal']

@update
def write_stable_principal(principal: Principal):
    stable_storage['stable_principal'] = principal

@query
def read_stable_user() -> User:
    return stable_storage['stable_user']

@update
def write_stable_user(user: User):
    stable_storage['stable_user'] = user

@query
def read_stable_reaction() -> Reaction:
    return stable_storage['stable_reaction']

@update
def write_stable_reaction(reaction: Reaction):
    stable_storage['stable_reaction'] = reaction

@query
def read_stable_func() -> StableFunc:
    return stable_storage['stable_func']

@update
def write_stable_func(func: StableFunc):
    stable_storage['stable_func'] = func

@query
def read_stable_boolean() -> bool:
    return stable_storage['stable_boolean']

@update
def write_stable_boolean(boolean: bool):
    stable_storage['stable_boolean'] = boolean

@query
def read_stable_null() -> null:
    return stable_storage['stable_null']

@update
def write_stable_null(null_: null):
    stable_storage['stable_null'] = null_

@query
def read_stable_opt() -> opt[int]:
    return stable_storage['stable_opt']

@update
def write_stable_opt(opt: opt[int]):
    stable_storage['stable_opt'] = opt
