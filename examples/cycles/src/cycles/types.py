from kybra import Canister, nat, nat64, Principal, update


class Cycles(Canister):
    @update
    def receive_cycles(self) -> nat64: ...

    @update
    def receive_cycles128(self) -> nat: ...


cycles = Cycles(Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai'))
