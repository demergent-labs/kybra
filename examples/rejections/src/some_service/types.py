from kybra import Canister, empty, method, Principal

class SomeService(Canister):
    @method
    def reject(self, message: str) -> empty: ...

    @method
    def accept(self) -> bool: ...

    @method
    def error(self) -> empty: ...

some_service = SomeService(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))
