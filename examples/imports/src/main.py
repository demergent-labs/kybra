from kybra import query
import import1
import import2
import import2.import3
import math
# from sample_adds.add import add_one #type: ignore TODO add an externally installed pip library in the future
import hashlib


@query
def get_one() -> str:
    return import1.get_message()


@query
def get_two() -> str:
    return import2.get_message()


@query
def get_three() -> str:
    return import2.import3.get_message()


@query
def sha224_hash(message: str) -> str:
    return hashlib.sha224(message.encode()).hexdigest()


@query
def get_math_message() -> int:
    return math.ceil(10.4)

# TODO add an externally installed pip library in the future
# @query
# def get_external_module_message() -> int:
#     return add_one(1, 2) #type: ignore
