# Copyright 2021 DFINITY Stiftung

# Licensed under the Apache License, Version 2.0 (the "License"); you may not use
# this file except in compliance with the License. You may obtain a copy of the
# License at

#    http://www.apache.org/licenses/LICENSE-2.0

# Unless required by applicable law or agreed to in writing, software distributed
# under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
# CONDITIONS OF ANY KIND, either express or implied. See the License for the
# specific language governing permissions and limitations under the License.

# The original license for the DFINITY code can be found here: https://github.com/dfinity/ic/blob/master/LICENSE
# This file contains derivative works licensed as MIT: https://github.com/demergent-labs/azle/blob/main/LICENSE

# Taken in part from: https://github.com/dfinity/interface-spec/blob/master/spec/ic.did

from kybra import blob, Canister, Principal, service_update, void

# The as expressions are reexporting these variables
from kybra.canisters.management.basic import (
    CanisterStatusResult as CanisterStatusResult,
    DepositCyclesArgs as DepositCyclesArgs,
    DeleteCanisterArgs as DeleteCanisterArgs,
    CreateCanisterArgs as CreateCanisterArgs,
    CreateCanisterResult as CreateCanisterResult,
    InstallCodeArgs as InstallCodeArgs,
    UninstallCodeArgs as UninstallCodeArgs,
    UpdateSettingsArgs as UpdateSettingsArgs,
    StartCanisterArgs as StartCanisterArgs,
    StopCanisterArgs as StopCanisterArgs,
    CanisterStatusArgs as CanisterStatusArgs,
    ProvisionalCreateCanisterWithCyclesArgs as ProvisionalCreateCanisterWithCyclesArgs,
    ProvisionalCreateCanisterWithCyclesResult as ProvisionalCreateCanisterWithCyclesResult,
    ProvisionalTopUpCanisterArgs as ProvisionalTopUpCanisterArgs,
)
from kybra.canisters.management.http import (
    HttpRequestArgs as HttpRequestArgs,
    HttpResponse as HttpResponse,
)
from kybra.canisters.management.bitcoin import (
    GetBalanceArgs as GetBalanceArgs,
    GetCurrentFeePercentilesArgs as GetCurrentFeePercentilesArgs,
    GetUtxosArgs as GetUtxosArgs,
    GetUtxosResult as GetUtxosResult,
    MillisatoshiPerByte as MillisatoshiPerByte,
    Satoshi as Satoshi,
    SendTransactionArgs as SendTransactionArgs,
)

# TODO change the return types to void


class ManagementCanister(Canister):
    @service_update
    def bitcoin_get_balance(self, args: GetBalanceArgs) -> Satoshi:
        ...

    @service_update
    def bitcoin_get_current_fee_percentiles(
        self, args: GetCurrentFeePercentilesArgs
    ) -> list[MillisatoshiPerByte]:
        ...

    @service_update
    def bitcoin_get_utxos(self, args: GetUtxosArgs) -> GetUtxosResult:
        ...

    @service_update
    def bitcoin_send_transaction(self, args: SendTransactionArgs) -> void:
        ...

    @service_update
    def create_canister(self, args: CreateCanisterArgs) -> CreateCanisterResult:
        ...

    @service_update
    def update_settings(self, args: UpdateSettingsArgs) -> void:
        ...

    @service_update
    def install_code(self, args: InstallCodeArgs) -> void:
        ...

    @service_update
    def uninstall_code(self, args: UninstallCodeArgs) -> void:
        ...

    @service_update
    def start_canister(self, args: StartCanisterArgs) -> void:
        ...

    @service_update
    def stop_canister(self, args: StopCanisterArgs) -> void:
        ...

    @service_update
    def canister_status(self, args: CanisterStatusArgs) -> CanisterStatusResult:
        ...

    @service_update
    def delete_canister(self, args: DeleteCanisterArgs) -> void:
        ...

    @service_update
    def deposit_cycles(self, args: DepositCyclesArgs) -> void:
        ...

    @service_update
    def raw_rand(self) -> blob:
        ...

    @service_update
    def provisional_create_canister_with_cycles(
        self, args: ProvisionalCreateCanisterWithCyclesArgs
    ) -> ProvisionalCreateCanisterWithCyclesResult:
        ...

    @service_update
    def provisional_top_up_canister(self, args: ProvisionalTopUpCanisterArgs) -> void:
        ...

    @service_update
    def http_request(self, args: HttpRequestArgs) -> HttpResponse:
        ...


management_canister = ManagementCanister(Principal.from_str("aaaaa-aa"))
