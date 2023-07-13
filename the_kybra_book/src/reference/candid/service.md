# service

This section is a work in progress.

Python classes that inherit from the Kybra type `Service` correspond to the [Candid service type](https://internetcomputer.org/docs/current/references/candid-ref#type-service-) and will become child classes capable of creating instances that can perform cross-canister calls at runtime.

Python:

```python
from kybra import (
    Async,
    CallResult,
    Principal,
    query,
    Service,
    service_query,
    service_update,
    update,
)


class SomeService(Service):
    @service_query
    def query1(self) -> bool:
        ...

    @service_update
    def update1(self) -> str:
        ...


@query
def get_service() -> SomeService:
    return SomeService(Principal.from_str("aaaaa-aa"))


@update
def call_service(service: SomeService) -> Async[str]:
    result: CallResult[str] = yield service.update1()

    if result.Err is not None:
        raise Exception(f"call to service.update1 failed with: {result.Err}")

    return result.Ok
```

Candid:

```
service { query1 : () -> (bool) query; update1 : () -> (text) }
```
