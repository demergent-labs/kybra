from kybra import Async, CanisterResult, nat64, NotifyResult, opt, Principal, update, Variant
from src.canister2.types import Account, AccountArgs, Canister2


class TransferResult(Variant, total=False):
    Ok: nat64
    Err: str


class BalanceResult(Variant, total=False):
    Ok: nat64
    Err: str


class AccountResult(Variant, total=False):
    Ok: opt[Account]
    Err: str


class AccountsResult(Variant, total=False):
    Ok: list[Account]
    Err: str


class TrapResult(Variant, total=False):
    Ok: str
    Err: str


canister2 = Canister2(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))


@update
def transfer(
    from_: str,
    to: str,
    amount: nat64
) -> Async[TransferResult]:
    result: CanisterResult[nat64] = yield canister2.transfer(from_, to, amount)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def balance(id: str) -> Async[BalanceResult]:
    result: CanisterResult[nat64] = yield canister2.balance(id)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def account(args: AccountArgs) -> Async[AccountResult]:
    result: CanisterResult[opt[Account]] = yield canister2.account(args)

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def accounts() -> Async[AccountsResult]:
    result: CanisterResult[list[Account]] = yield canister2.accounts()

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def trap() -> Async[TrapResult]:
    result: CanisterResult[str] = yield canister2.trap()

    if result.Err is not None:
        return {
            'Err': result.Err
        }

    return {
        'Ok': result.Ok
    }


@update
def send_notification() -> NotifyResult:
    return canister2.receive_notification('This is the notification').notify()
