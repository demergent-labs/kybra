from kybra import null, Variant
from kybra.canisters.management import GetUtxosResult, MillisatoshiPerByte, Satoshi


class ExecuteGetBalanceResult(Variant, total=False):
    Ok: Satoshi
    Err: str


class ExecuteGetCurrentFeePercentiles(Variant, total=False):
    Ok: list[MillisatoshiPerByte]
    Err: str


class ExecuteGetUtxosResult(Variant, total=False):
    Ok: GetUtxosResult
    Err: str


class ExecuteSendTransactionResult(Variant, total=False):
    Ok: null
    Err: str
