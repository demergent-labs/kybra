from kybra import (
    Async,
    ic,
    init,
    Principal,
    RejectionCode,
    Service,
    service_update,
    update,
    void,
)
from src.some_service.types import SomeService


class Nonexistent(Service):
    @service_update
    def method(self) -> void:
        ...


some_service = SomeService(Principal.from_str("aaaaa-aa"))
nonexistent_canister = Nonexistent(Principal.from_str("rkp4c-7iaaa-aaaaa-aaaca-cai"))


@init
def init_(some_service_id: Principal) -> void:
    global some_service
    some_service = SomeService(some_service_id)


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
    yield some_service.reject("reject")
    return ic.reject_code()


@update
def get_rejection_code_canister_error() -> Async[RejectionCode]:
    yield some_service.error()
    return ic.reject_code()


@update
def get_rejection_message(message: str) -> Async[str]:
    yield some_service.reject(message)
    return ic.reject_message()
