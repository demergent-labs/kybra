from kybra import Canister, method, nat, nat64, Principal

class Cycles(Canister):
    @method
    def receive_cycles(self) -> nat64: ...

    @method
    def receive_cycles128(self) -> nat: ...

cycles = Cycles(Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai'))
