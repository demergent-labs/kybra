from kybra import Canister, method, nat, Variant


class Canister1(Canister):
    @method
    def inc_counter(self) -> nat: ...


class StringQueryResult(Variant, total=False):
    ok: str
    err: str


class NatQueryResult(Variant, total=False):
    ok: nat
    err: str
