from kybra import nat, Service, service_query, Variant


class Canister1(Service):
    @service_query
    def inc_counter(self) -> nat: ...


class StringQueryResult(Variant, total=False):
    Ok: str
    Err: str


class NatQueryResult(Variant, total=False):
    Ok: nat
    Err: str
