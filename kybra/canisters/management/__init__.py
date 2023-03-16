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

from kybra import blob, Canister, Principal, update, void

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
    @update
    def bitcoin_get_balance(self, args: GetBalanceArgs) -> Satoshi:
        ...

    @update
    def bitcoin_get_current_fee_percentiles(
        self, args: GetCurrentFeePercentilesArgs
    ) -> list[MillisatoshiPerByte]:
        ...

    @update
    def bitcoin_get_utxos(self, args: GetUtxosArgs) -> GetUtxosResult:
        ...

    @update
    def bitcoin_send_transaction(self, args: SendTransactionArgs) -> void:
        ...

    @update
    def create_canister(self, args: CreateCanisterArgs) -> CreateCanisterResult:
        ...

    @update
    def update_settings(self, args: UpdateSettingsArgs) -> void:
        ...

    @update
    def install_code(self, args: InstallCodeArgs) -> void:
        ...

    @update
    def uninstall_code(self, args: UninstallCodeArgs) -> void:
        ...

    @update
    def start_canister(self, args: StartCanisterArgs) -> void:
        ...

    @update
    def stop_canister(self, args: StopCanisterArgs) -> void:
        ...

    @update
    def canister_status(self, args: CanisterStatusArgs) -> CanisterStatusResult:
        ...

    @update
    def delete_canister(self, args: DeleteCanisterArgs) -> void:
        ...

    @update
    def deposit_cycles(self, args: DepositCyclesArgs) -> void:
        ...

    @update
    def raw_rand(self) -> blob:
        ...

    @update
    def provisional_create_canister_with_cycles(
        self, args: ProvisionalCreateCanisterWithCyclesArgs
    ) -> ProvisionalCreateCanisterWithCyclesResult:
        ...

    @update
    def provisional_top_up_canister(self, args: ProvisionalTopUpCanisterArgs) -> void:
        ...

    @update
    def http_request(self, args: HttpRequestArgs) -> HttpResponse:
        ...


management_canister = ManagementCanister(Principal.from_str("aaaaa-aa"))
