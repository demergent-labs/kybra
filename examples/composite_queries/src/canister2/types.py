from kybra import Canister, nat, query, update
from src.canister1.types import StringQueryResult


class Canister2(Canister):
    @query
    def simple_query(self) -> str: ...

    @query
    def manual_query(self) -> str: ...

    @update
    def update_query(self) -> str: ...

    @query
    def deep_query(self) -> StringQueryResult: ...

    @query
    def inc_counter(self) -> nat: ...
