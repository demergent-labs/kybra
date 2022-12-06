# TODO use enum when standard lib is supported
# from enum import Enum


# class OrderType(Enum):
#     Less = -1
#     Equal = 0
#     Greater = 1


Less = -1
Equal = 0
Greater = 1


def compare(a: int, b: int) -> int:
    if a > b:
        return Greater
    if a < b:
        return Less
    return Equal


def isLess(order: int) -> bool:
    return order == Less


def isEqual(order: int) -> bool:
    return order == Equal


def isGreater(order: int) -> bool:
    return order == Greater
