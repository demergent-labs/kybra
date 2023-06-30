from kybra import (
    Async,
    CallResult,
    init,
    match,
    nat64,
    NotifyResult,
    Opt,
    Principal,
    update,
    Variant,
    Vec,
    void,
)
from src.canister2.types import Account, AccountArgs, Canister2


class TransferResult(Variant, total=False):
    Ok: nat64
    Err: str


class BalanceResult(Variant, total=False):
    Ok: nat64
    Err: str


class AccountResult(Variant, total=False):
    Ok: Opt[Account]
    Err: str


class AccountsResult(Variant, total=False):
    Ok: Vec[Account]
    Err: str


class TrapResult(Variant, total=False):
    Ok: str
    Err: str


canister2 = Canister2(Principal.from_str(""))


@init
def init_(canister2_id: Principal) -> void:
    global canister2
    canister2 = Canister2(canister2_id)


@update
def transfer(from_: str, to: str, amount: nat64) -> Async[TransferResult]:
    result: CallResult[nat64] = yield canister2.transfer(from_, to, amount)

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def balance(id: str) -> Async[BalanceResult]:
    result: CallResult[nat64] = yield canister2.balance(id)

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def account(args: AccountArgs) -> Async[AccountResult]:
    result: CallResult[Opt[Account]] = yield canister2.account(args)

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def accounts() -> Async[AccountsResult]:
    result: CallResult[Vec[Account]] = yield canister2.accounts()

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def trap() -> Async[TrapResult]:
    result: CallResult[str] = yield canister2.trap()

    return match(
        result,
        {
            "Ok": lambda ok: {"Ok": ok},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def send_notification() -> NotifyResult:
    return canister2.receive_notification("This is the notification").notify()
