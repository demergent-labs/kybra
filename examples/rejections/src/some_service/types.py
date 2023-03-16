from kybra import Canister, empty, Principal, query


class SomeService(Canister):
    @query
    def reject(self, message: str) -> empty: ...

    @query
    def accept(self) -> bool: ...

    @query
    def error(self) -> empty: ...


some_service = SomeService(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))
