from kybra import nat, nat64, Service, service_update


class Cycles(Service):
    @service_update
    def receive_cycles(self) -> nat64:
        ...

    @service_update
    def receive_cycles128(self) -> nat:
        ...
