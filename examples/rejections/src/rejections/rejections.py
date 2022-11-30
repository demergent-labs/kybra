from kybra import Async, Canister, ic, method, Principal, RejectionCode, update
from src.some_service.types import some_service

class Nonexistent(Canister):
    @method
    def method(self) -> None: ...

nonexistent_canister = Nonexistent(Principal.from_str('rkp4c-7iaaa-aaaaa-aaaca-cai'))

@update
def get_rejection_code_no_error() -> Async[RejectionCode]:
    yield some_service.accept()
    return ic.reject_code()

@update
def get_rejection_code_destination_invalid() -> Async[RejectionCode]:
    yield nonexistent_canister.method()
    return ic.reject_code()

@update
def get_rejection_code_canister_reject() -> Async[RejectionCode]:
    yield some_service.reject('reject')
    return ic.reject_code()

@update
def get_rejection_code_canister_error() -> Async[RejectionCode]:
    yield some_service.error()
    return ic.reject_code()

@update
def get_rejection_message(message: str) -> Async[str]:
    yield some_service.reject(message)
    return ic.reject_message()
