from kybra import blob, Canister, method, Principal

# TODO working on the management canister

class ManagementCanister(Canister):
    @method
    def raw_rand(self) -> blob: ...

management_canister = ManagementCanister(Principal.from_str('aaaaa-aa'))
