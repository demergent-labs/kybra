# type: ignore
# This file has a bunch of expressions that do not get the token length
# calculated correctly and therefore would not be properly highlighted in
# errors.


# These three bring in a lot of stuff that doesn't work and is enumerated bellow
# from kybra import query, opt, text, Record
# import kybra as kybra2
# from kybra import thing


BasicFunc: TypeAlias = Func(Query[[str], str])


# Function with matching inside
def my_function_2_25(arg1):
    match 3:
        case 1:
            print()
        case 2:
            print()
        case 3:
            print()
        case _:
            print()


# TODO Match don't work
match 3:
    case 1:
        print()
    case 2:
        print()
    case 3:
        print()
    case _:
        print()


# TODO this is going to be more challenging and I'll come back to it later.
# Use an attribute reference to access an attribute of an object
"hello".upper()


# Make sure that f strings work
class c3:
    def function_with_f_string(self, person: str) -> str:
        return f"Hello {person}"


# bstring statement
# TODO
_b32alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"


# bstring and fstring if a function
# TODO
def function_with_f_and_b_string():
    bstring = b"==="
    fstring = f"Hello {bstring}"
    print(fstring)


# It doesn't catch the very first quote nor the last three
"""
    # !@#$!@#$!@#$$#%^$^&^%&*^&^()*()&(^*&$%^&#$#%#@)
    thing = 1 ** 2
    thing = [[1]] @ [[2]]
    thing = 1 + 2
    thing = 1 ^ 2
    thing = 1 - 1
    thing = 1 * 1
    thing = 1 / 1
    thing = 1 % 1
    thing = 1 << 1
    thing = 1 >> 1
    thing = 1 | 1
    thing = 1 & 1
    print(thing)
"""


# # TODO The canister results with the ellipsis aren't quite right
# # Handle the ellipsis
class CanisterResult(Generic[T]):
    ok: T
    err: Optional[str]

    def __init__(self, ok: T, err: str):
        self.ok = ok
        self.err = err

    def notify(self) -> "NotifyResult":
        ...

    def with_cycles(self, cycles: nat64) -> "CanisterResult[T]":
        ...

    def with_cycles128(self, cycles: nat) -> "CanisterResult[T]":
        ...


# # TODO The IC object is also not working.
