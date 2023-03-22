from kybra import Async, blob, CanisterResult, update, void
from kybra.canisters.management import GetUtxosResult, management_canister, MillisatoshiPerByte, Satoshi
from bitcoin_types import ExecuteGetBalanceResult, ExecuteGetCurrentFeePercentiles, ExecuteGetUtxosResult, ExecuteSendTransactionResult

BITCOIN_API_CYCLE_COST = 100_000_000
BITCOIN_BASE_TRANSACTION_COST = 5_000_000_000
BITCOIN_CYCLE_COST_PER_TRANSACTION_BYTE = 20_000_000


@update
def get_balance(address: str) -> Async[ExecuteGetBalanceResult]:
    canister_result: CanisterResult[Satoshi] = yield management_canister.bitcoin_get_balance(
        {
            'address': address,
            'min_confirmations': None,
            'network': {'Regtest': None}
        }
    ).with_cycles(BITCOIN_API_CYCLE_COST)

    if canister_result.Err is not None:
        return {
            'Err': canister_result.Err
        }

    return {'Ok': canister_result.Ok}


@update
def get_current_fee_percentiles() -> Async[ExecuteGetCurrentFeePercentiles]:
    canister_result: CanisterResult[list[MillisatoshiPerByte]] = yield management_canister.bitcoin_get_current_fee_percentiles({
        'network': {'Regtest': None}
    }).with_cycles(BITCOIN_API_CYCLE_COST)

    if canister_result.Err is not None:
        return {
            'Err': canister_result.Err
        }

    return {'Ok': canister_result.Ok}


@update
def get_utxos(address: str) -> Async[ExecuteGetUtxosResult]:
    canister_result: CanisterResult[GetUtxosResult] = yield management_canister.bitcoin_get_utxos({
        'address': address,
        'filter': None,
        'network': {'Regtest': None}
    }).with_cycles(BITCOIN_API_CYCLE_COST)

    if canister_result.Err is not None:
        return {
            'Err': canister_result.Err
        }

    return {'Ok': canister_result.Ok}


@update
def send_transaction(
    transaction: blob
) -> Async[ExecuteSendTransactionResult]:
    transaction_fee = BITCOIN_BASE_TRANSACTION_COST + \
        len(transaction) * BITCOIN_CYCLE_COST_PER_TRANSACTION_BYTE

    canister_result: CanisterResult[void] = yield management_canister.bitcoin_send_transaction({
        'transaction': transaction,
        'network': {'Regtest': None}
    }).with_cycles(transaction_fee)

    if canister_result.Err is not None:
        return {
            'Err': canister_result.Err
        }

    return {'Ok': canister_result.Ok}
