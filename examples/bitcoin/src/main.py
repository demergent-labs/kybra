from kybra import Async, blob, CallResult, match, update, void
from kybra.canisters.management import (
    GetUtxosResult,
    management_canister,
    MillisatoshiPerByte,
    Satoshi,
)
from bitcoin_types import (
    ExecuteGetBalanceResult,
    ExecuteGetCurrentFeePercentiles,
    ExecuteGetUtxosResult,
    ExecuteSendTransactionResult,
)

BITCOIN_API_CYCLE_COST = 100_000_000
BITCOIN_BASE_TRANSACTION_COST = 5_000_000_000
BITCOIN_CYCLE_COST_PER_TRANSACTION_BYTE = 20_000_000


@update
def get_balance(address: str) -> Async[ExecuteGetBalanceResult]:
    call_result: CallResult[Satoshi] = yield management_canister.bitcoin_get_balance(
        {"address": address, "min_confirmations": None, "network": {"Regtest": None}}
    ).with_cycles(BITCOIN_API_CYCLE_COST)

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )


@update
def get_current_fee_percentiles() -> Async[ExecuteGetCurrentFeePercentiles]:
    call_result: CallResult[
        list[MillisatoshiPerByte]
    ] = yield management_canister.bitcoin_get_current_fee_percentiles(
        {"network": {"Regtest": None}}
    ).with_cycles(
        BITCOIN_API_CYCLE_COST
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )


@update
def get_utxos(address: str) -> Async[ExecuteGetUtxosResult]:
    call_result: CallResult[
        GetUtxosResult
    ] = yield management_canister.bitcoin_get_utxos(
        {"address": address, "filter": None, "network": {"Regtest": None}}
    ).with_cycles(
        BITCOIN_API_CYCLE_COST
    )

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )


@update
def send_transaction(transaction: blob) -> Async[ExecuteSendTransactionResult]:
    transaction_fee = (
        BITCOIN_BASE_TRANSACTION_COST
        + len(transaction) * BITCOIN_CYCLE_COST_PER_TRANSACTION_BYTE
    )

    call_result: CallResult[void] = yield management_canister.bitcoin_send_transaction(
        {"transaction": transaction, "network": {"Regtest": None}}
    ).with_cycles(transaction_fee)

    return match(
        call_result,
        {"Ok": lambda ok: {"Ok": True}, "Err": lambda err: {"Err": err}},
    )
