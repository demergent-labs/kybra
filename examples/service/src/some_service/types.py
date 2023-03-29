from kybra import Principal, Service, service_query, service_update


class SomeService(Service):
    @service_query
    def query1(self) -> bool: ...

    @service_update
    def update1(self) -> str: ...


some_service = SomeService(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))
