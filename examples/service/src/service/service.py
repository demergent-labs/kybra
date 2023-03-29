from kybra import Async, CanisterResult, query, update, Variant
from src.some_service.types import some_service, SomeService


class Update1Result(Variant, total=False):
    Ok: str
    Err: str


@query
def service_param(some_service: SomeService) -> SomeService:
    return some_service


@query
def service_return_type() -> SomeService:
    return some_service


@update
def service_list(some_services: list[SomeService]) -> list[SomeService]:
    return some_services


@update
def service_cross_canister_call(some_service: SomeService) -> Async[Update1Result]:
    result: CanisterResult[str] = yield some_service.update1()

    if result.Err is not None:
        return {"Err": result.Err}

    return {"Ok": result.Ok}
