from kybra import blob, Variant
from kybra.canisters.management import CreateCanisterResult, CanisterStatusResult

class DefaultResult(Variant, total=False):
    ok: bool
    err: str

class ExecuteCreateCanisterResult(Variant, total=False):
    ok: CreateCanisterResult
    err: str

class ExecuteProvisionalCreateCanisterWithCyclesResult(Variant, total=False):
    ok: CreateCanisterResult
    err: str

class GetCanisterStatusResult(Variant, total=False):
    ok: CanisterStatusResult
    err: str

class RawRandResult(Variant, total=False):
    ok: blob
    err: str
