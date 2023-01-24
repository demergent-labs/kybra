from kybra import Canister, method, nat
from src.canister1.types import StringQueryResult


class Canister2(Canister):
    @method
    def simple_query(self) -> str: ...

    @method
    def manual_query(self) -> str: ...

    @method
    def update_query(self) -> str: ...

    @method
    def deep_query(self) -> StringQueryResult: ...

    @method
    def inc_counter(self) -> nat: ...
