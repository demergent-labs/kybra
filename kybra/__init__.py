# from typing import Optional, TypeVar, TypedDict, Union
# from enum import Enum

int64 = int
int32 = int
int16 = int
int8 = int

nat = int
nat64 = int
nat32 = int
nat16 = int
nat8 = int

float64 = float
float32 = float

text = str

# blob = bytes

# null = None

# my_list = list[int]

# T = TypeVar('T')
# opt = Optional[T]

# list[T]

# Variant = Enum
# Record = TypedDict

# my_tuple = tuple[str, str]

# class MyRecord(TypedDict):
#     prop1: str
#     prop2: int

# class MyVariant1:
#     prop1: str

# class MyVariant2:
#     prop2: int

# variant = MyVariant1 | MyVariant2

# your_variant: variant = MyVariant1()

# if your_variant.prop2

# def test(the_variant: variant):
#     return the_variant.prop2

# class MyVariant:
#     prop1: str
#     prop2: int

# my_variant: MyVariant = MyVariant()

# class MyRecord(Record):
#     prop1: str
#     prop2: int

# class MyVariant(Variant):
#     Prop1 = opt[str]
#     Prop2 = opt[int]

# TODO this should work for records
# MyRecord = Record('MyRecord', {
#     'prop1': str,
#     'prop2': int
# })

# TODO we just need to figure out how to get the typechecking to work well on the variant
# TODO if I can make my own class called variant that takes in these types as params and sets them to opt, we might be good to go
# TODO also look into an enum
# MyVariant = Variant('MyVariant', {
#     'prop1': opt[str],
#     'prop2': opt[int]
# })

# my_record = dict[
#     'hello': int32
# ]

# object

# TODO we probably want to use decorators for variants and records

def query(func: object):
    return func

def update(func: object):
    return func

# def record(func: object):
#     return func

# def variant(func: object):
#     return func

# def test_variant(my_variant: MyVariant) -> MyVariant:

#     # TODO working on the Enum, we need it to carry a dynamic value
#     another_variant: MyVariant = MyVariant(Prop1 = 'hello')

#     return my_variant.Prop2

# def monkey(record: MyRecord) -> str:
#     temp: MyRecord = {
#         'prop1': 'yes',
#         'prop2': 32
#     }

#     # temp: MyRecord = new MyRecord()

#     return temp['prop1']
