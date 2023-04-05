from kybra import nat, Service, service_query, service_update
from src.canister1.types import StringQueryResult


class Canister2(Service):
    @service_query
    def simple_query(self) -> str: ...

    @service_query
    def manual_query(self) -> str: ...

    @service_update
    def update_query(self) -> str: ...

    @service_query
    def deep_query(self) -> StringQueryResult: ...

    @service_query
    def inc_counter(self) -> nat: ...
