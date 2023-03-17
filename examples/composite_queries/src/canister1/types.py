from kybra import Canister, nat, service_query, Variant


class Canister1(Canister):
    @service_query
    def inc_counter(self) -> nat: ...


class StringQueryResult(Variant, total=False):
    ok: str
    err: str


class NatQueryResult(Variant, total=False):
    ok: nat
    err: str
