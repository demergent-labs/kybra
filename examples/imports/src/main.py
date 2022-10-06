from kybra import query
import import1
import import2
import import2.import3
import math
from sample_adds.add import add_one #type: ignore

@query
def get_import1_message() -> str:
    return import1.get_message()

@query
def get_import2_message() -> str:
    return import2.get_message()

@query
def get_import3_message() -> str:
    return import2.import3.get_message()

@query
def get_math_message() -> int:
    return math.ceil(10.4)

@query
def get_external_module_message() -> int:
    return add_one(1, 2) #type: ignore
