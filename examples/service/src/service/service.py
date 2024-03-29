from kybra import (
    Async,
    CallResult,
    Principal,
    query,
    Service,
    service_query,
    service_update,
    update,
    Variant,
    Vec,
)


class SomeService(Service):
    @service_query
    def query1(self) -> bool:
        ...

    @service_update
    def update1(self) -> str:
        ...


some_service = SomeService(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))


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
def service_list(some_services: Vec[SomeService]) -> Vec[SomeService]:
    return some_services


@update
def service_cross_canister_call(some_service: SomeService) -> Async[Update1Result]:
    result: CallResult[str] = yield some_service.update1()

    if result.Err is not None:
        return {"Err": result.Err}

    return {"Ok": result.Ok}
