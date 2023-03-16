from kybra import Canister, query


class Canister3(Canister):
    @query
    def deep_query(self) -> str: ...
