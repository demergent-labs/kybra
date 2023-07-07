from kybra import empty, Service, service_query


class SomeService(Service):
    @service_query
    def reject(self, message: str) -> empty:
        ...

    @service_query
    def accept(self) -> bool:
        ...

    @service_query
    def error(self) -> empty:
        ...
