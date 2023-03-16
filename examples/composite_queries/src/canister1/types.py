from kybra import Canister, nat, query, Variant


class Canister1(Canister):
    @query
    def inc_counter(self) -> nat: ...


class StringQueryResult(Variant, total=False):
    ok: str
    err: str


class NatQueryResult(Variant, total=False):
    ok: nat
    err: str
