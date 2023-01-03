from kybra import Canister, method


class Canister3(Canister):
    @method
    def deep_query(self) -> str: ...
