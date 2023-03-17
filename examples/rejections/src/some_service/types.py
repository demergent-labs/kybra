from kybra import Canister, empty, Principal, service_query


class SomeService(Canister):
    @service_query
    def reject(self, message: str) -> empty: ...

    @service_query
    def accept(self) -> bool: ...

    @service_query
    def error(self) -> empty: ...


some_service = SomeService(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))
