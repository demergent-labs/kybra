from kybra import Service, service_query


class Canister3(Service):
    @service_query
    def deep_query(self) -> str: ...
