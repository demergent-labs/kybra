from kybra import Canister, service_query


class Canister3(Canister):
    @service_query
    def deep_query(self) -> str: ...
