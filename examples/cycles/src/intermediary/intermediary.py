from kybra import (
    Async,
    CanisterResult,
    ic,
    match,
    nat,
    nat64,
    NotifyResult,
    query,
    update,
    Variant,
)
from src.cycles.types import cycles


class SendCyclesResult(Variant, total=False):
    Ok: nat64
    Err: str


class SendCyclesResult128(Variant, total=False):
    Ok: nat
    Err: str


# Reports the number of cycles returned from the Cycles canister


@update
def send_cycles() -> Async[SendCyclesResult]:
    result: CanisterResult[nat64] = yield cycles.receive_cycles().with_cycles(1_000_000)

    return match(
        result,
        {
            "Ok": lambda _: {"Ok": ic.msg_cycles_refunded()},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def send_cycles_notify() -> NotifyResult:
    return cycles.receive_cycles().with_cycles(1_000_000).notify()


# Reports the number of cycles returned from the Cycles canister


@update
def send_cycles128() -> Async[SendCyclesResult128]:
    result: CanisterResult[nat] = yield cycles.receive_cycles128().with_cycles128(
        1_000_000
    )

    return match(
        result,
        {
            "Ok": lambda _: {"Ok": ic.msg_cycles_refunded128()},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def send_cycles128_notify() -> NotifyResult:
    return cycles.receive_cycles128().with_cycles128(1_000_000).notify()


@query
def get_canister_balance() -> nat64:
    return ic.canister_balance()


@query
def get_canister_balance128() -> nat:
    return ic.canister_balance128()
