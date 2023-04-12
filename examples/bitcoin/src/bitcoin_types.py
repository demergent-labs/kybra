from kybra import Variant, Vec
from kybra.canisters.management import GetUtxosResult, MillisatoshiPerByte, Satoshi


class ExecuteGetBalanceResult(Variant, total=False):
    Ok: Satoshi
    Err: str


class ExecuteGetCurrentFeePercentiles(Variant, total=False):
    Ok: Vec[MillisatoshiPerByte]
    Err: str


class ExecuteGetUtxosResult(Variant, total=False):
    Ok: GetUtxosResult
    Err: str


class ExecuteSendTransactionResult(Variant, total=False):
    Ok: bool
    Err: str
