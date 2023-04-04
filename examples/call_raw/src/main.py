from kybra import (
    Async,
    blob,
    CallResult,
    ic,
    match,
    nat,
    nat64,
    Principal,
    update,
    Variant,
)


class ExecuteCallRawResult(Variant, total=False):
    Ok: str
    Err: str


@update
def execute_call_raw(
    canister_id: Principal, method: str, candid_args: str, payment: nat64
) -> Async[ExecuteCallRawResult]:
    canister_result: CallResult[blob] = yield ic.call_raw(
        canister_id, method, ic.candid_encode(candid_args), payment
    )

    return match(
        canister_result,
        {
            "Ok": lambda ok: {"Ok": ic.candid_decode(ok)},
            "Err": lambda err: {"Err": err},
        },
    )


class ExecuteCallRaw128Result(Variant, total=False):
    Ok: str
    Err: str


@update
def execute_call_raw128(
    canister_id: Principal, method: str, candid_args: str, payment: nat
) -> Async[ExecuteCallRaw128Result]:
    canister_result: CallResult[blob] = yield ic.call_raw128(
        canister_id, method, ic.candid_encode(candid_args), payment
    )

    return match(
        canister_result,
        {
            "Ok": lambda ok: {"Ok": ic.candid_decode(ok)},
            "Err": lambda err: {"Err": err},
        },
    )
