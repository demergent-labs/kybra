from kybra import blob, Variant
from kybra.canisters.management import CanisterStatusResult, CreateCanisterResult


class DefaultResult(Variant, total=False):
    Ok: bool
    Err: str


class ExecuteCreateCanisterResult(Variant, total=False):
    Ok: CreateCanisterResult
    Err: str


class ExecuteProvisionalCreateCanisterWithCyclesResult(Variant, total=False):
    Ok: CreateCanisterResult
    Err: str


class GetCanisterStatusResult(Variant, total=False):
    Ok: CanisterStatusResult
    Err: str


class RawRandResult(Variant, total=False):
    Ok: blob
    Err: str
