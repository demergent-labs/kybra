from kybra import blob, Canister, method, Principal
from kybra.canisters.management.basic import CanisterStatusResult, DepositCyclesArgs, DeleteCanisterArgs, CreateCanisterArgs, CreateCanisterResult, InstallCodeArgs, UninstallCodeArgs, UpdateSettingsArgs, StartCanisterArgs, StopCanisterArgs, CanisterStatusArgs, ProvisionalCreateCanisterWithCyclesArgs, ProvisionalCreateCanisterWithCyclesResult, ProvisionalTopUpCanisterArgs

# TODO change the return types to void

class ManagementCanister(Canister):
    @method
    def create_canister(self, args: CreateCanisterArgs) -> CreateCanisterResult: ...

    @method
    def update_settings(self, args: UpdateSettingsArgs) -> None: ...

    @method
    def install_code(self, args: InstallCodeArgs) -> None: ...

    @method
    def uninstall_code(self, args: UninstallCodeArgs) -> None: ...

    @method
    def start_canister(self, args: StartCanisterArgs) -> None: ...

    @method
    def stop_canister(self, args: StopCanisterArgs) -> None: ...

    @method
    def canister_status(self, args: CanisterStatusArgs) -> CanisterStatusResult: ...

    @method
    def delete_canister(self, args: DeleteCanisterArgs) -> None: ...

    @method
    def deposit_cycles(self, args: DepositCyclesArgs) -> None: ...

    @method
    def raw_rand(self) -> blob: ...

    @method
    def provisional_create_canister_with_cycles(self, args: ProvisionalCreateCanisterWithCyclesArgs) -> ProvisionalCreateCanisterWithCyclesResult: ...

    @method
    def provisional_top_up_canister(self, args: ProvisionalTopUpCanisterArgs) -> None: ...

management_canister = ManagementCanister(Principal.from_str('aaaaa-aa'))
