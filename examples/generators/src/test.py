from typing import Any, cast, TypeVar

# TODO we really need to use the ManagementCanister to get this to work appropriately, because of the candid decoding
# TODO swap out ic.call_raw for the ManagementCanister once we have general cross-canister calls enabled

default = cast(Any, '')

T = TypeVar('T')

# TODO I wonder if we can loop through each method defined by the user and manually override


class Service:
    canister_id: str

    def __init__(self, canister_id: str):
        self.canister_id = canister_id

# TODO https://stackoverflow.com/questions/2704434/intercept-method-calls-in-python
# TODO try to get rid of the need for the call decorator by iterating over the methods in the superclass
# TODO watch out for *kwargs


def call(func: T) -> T:
    def intermediate_func(*args):  # type: ignore
        the_self = args[0]  # type: ignore
        selfless_args = args[1:]  # type: ignore

        return {
            'name': 'call',
            'args': [
                func.__qualname__,  # type: ignore
                the_self.canister_id,  # type: ignore
                *selfless_args
            ]
        }  # type: ignore

    return intermediate_func  # type: ignore

# def staticmethod():
#     pass

# def test(func: T) -> staticmethod[T]:
#     return staticmethod(func)

# class ManagementCanister(Service):
#     # @staticmethod
#     @cross_canister_call
#     def raw_rand(string: str) -> bytes: ...

# management_canister = ManagementCanister('aaaaa-aa')

# canister_result = management_canister.raw_rand('monkey')

# print(canister_result)


CanisterResult = tuple


class ManagementCanister(Service):
    @call
    def raw_rand(self) -> CanisterResult[bytes, str]: ...


management_canister = ManagementCanister('aaaaa-aa')

canister_result = management_canister.raw_rand()
