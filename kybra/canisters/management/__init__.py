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

from kybra import blob, Canister, method, Principal
from kybra.canisters.management.basic import CanisterStatusResult, DepositCyclesArgs, DeleteCanisterArgs, CreateCanisterArgs, CreateCanisterResult, InstallCodeArgs, UninstallCodeArgs, UpdateSettingsArgs, StartCanisterArgs, StopCanisterArgs, CanisterStatusArgs, ProvisionalCreateCanisterWithCyclesArgs, ProvisionalCreateCanisterWithCyclesResult, ProvisionalTopUpCanisterArgs
from kybra.canisters.management.http import HttpRequestArgs, HttpResponse
from kybra.canisters.management.bitcoin import GetBalanceArgs, GetCurrentFeePercentilesArgs, GetUtxosArgs, GetUtxosResult, MillisatoshiPerByte, Satoshi, SendTransactionArgs

# TODO change the return types to void


class ManagementCanister(Canister):
    @method
    def bitcoin_get_balance(self, args: GetBalanceArgs) -> Satoshi: ...

    @method
    def bitcoin_get_current_fee_percentiles(
        self, args: GetCurrentFeePercentilesArgs) -> list[MillisatoshiPerByte]: ...

    @method
    def bitcoin_get_utxos(self, args: GetUtxosArgs) -> GetUtxosResult: ...

    @method
    def bitcoin_send_transaction(self, args: SendTransactionArgs) -> None: ...

    @method
    def create_canister(
        self, args: CreateCanisterArgs) -> CreateCanisterResult: ...

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
    def canister_status(
        self, args: CanisterStatusArgs) -> CanisterStatusResult: ...

    @method
    def delete_canister(self, args: DeleteCanisterArgs) -> None: ...

    @method
    def deposit_cycles(self, args: DepositCyclesArgs) -> None: ...

    @method
    def raw_rand(self) -> blob: ...

    @method
    def provisional_create_canister_with_cycles(
        self, args: ProvisionalCreateCanisterWithCyclesArgs) -> ProvisionalCreateCanisterWithCyclesResult: ...

    @method
    def provisional_top_up_canister(
        self, args: ProvisionalTopUpCanisterArgs) -> None: ...

    @method
    def http_request(self, args: HttpRequestArgs) -> HttpResponse: ...


management_canister = ManagementCanister(Principal.from_str('aaaaa-aa'))
