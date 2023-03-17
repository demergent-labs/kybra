from kybra import Canister, nat, nat64, Principal, service_update


class Cycles(Canister):
    @service_update
    def receive_cycles(self) -> nat64: ...

    @service_update
    def receive_cycles128(self) -> nat: ...


cycles = Cycles(Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai'))
