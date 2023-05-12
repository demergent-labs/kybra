import hashlib
import math

import boltons.mathutils
from kybra import float64, query

import import1
import import2
import import2.import3


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


@query
def boltons_floor(number: float64) -> int:
    return boltons.mathutils.floor(number)  # type: ignore
