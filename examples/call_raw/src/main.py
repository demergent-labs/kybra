from kybra import Async, blob, CanisterResult, ic, nat, nat64, Principal, update, Variant

class ExecuteCallRawResult(Variant, total=False):
    ok: str
    err: str

@update
def execute_call_raw(
    canister_id: Principal,
    method: str,
    candid_args: str,
    payment: nat64
) -> Async[ExecuteCallRawResult]:
    canister_result: CanisterResult[blob] = yield ic.call_raw(
        canister_id,
        method,
        ic.candid_encode(candid_args),
        payment
    )

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': ic.candid_decode(canister_result.ok)
    }

class ExecuteCallRaw128Result(Variant, total=False):
    ok: str
    err: str

@update
def execute_call_raw128(
    canister_id: Principal,
    method: str,
    candid_args: str,
    payment: nat
) -> Async[ExecuteCallRawResult]:
    canister_result: CanisterResult[blob] = yield ic.call_raw128(
        canister_id,
        method,
        ic.candid_encode(candid_args),
        payment
    )

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': ic.candid_decode(canister_result.ok)
    }
