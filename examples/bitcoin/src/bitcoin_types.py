from kybra import null, Variant
from kybra.canisters.management.bitcoin import GetUtxosResult, MillisatoshiPerByte, Satoshi


class ExecuteGetBalanceResult(Variant, total=False):
    ok: Satoshi
    err: str


class ExecuteGetCurrentFeePercentiles(Variant, total=False):
    ok: list[MillisatoshiPerByte]
    err: str


class ExecuteGetUtxosResult(Variant, total=False):
    ok: GetUtxosResult
    err: str


class ExecuteSendTransactionResult(Variant, total=False):
    ok: null
    err: str
