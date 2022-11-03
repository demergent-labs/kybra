from kybra import Async, CanisterResult, nat64, opt, Principal, update, Variant
from ..canister2.types import Account, AccountArgs, Canister2

class TransferResult(Variant):
    ok: nat64
    err: str

class BalanceResult(Variant):
    ok: nat64
    err: str

class AccountResult(Variant):
    ok: opt[Account]
    err: str

class AccountsResult(Variant):
    ok: list[Account]
    err: str

class TrapResult(Variant):
    ok: str
    err: str

class NotifyResult(Variant):
    ok: None
    err: str

canister2 = Canister2(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))

@update
def transfer(
    from_: str,
    to: str,
    amount: nat64
) -> Async[TransferResult]:
    result: CanisterResult[nat64] = yield canister2.transfer(from_, to, amount)

    return {
        'ok': result.ok,
        'err': result.err
    }

@update
def balance(id: str) -> Async[BalanceResult]:
    result: CanisterResult[nat64] = yield canister2.balance(id)

    return {
        'ok': result.ok,
        'err': result.err
    }

@update
def account(args: AccountArgs) -> Async[AccountResult]:
    result: CanisterResult[opt[Account]] = yield canister2.account(args)

    return {
        'ok': result.ok,
        'err': result.err
    }

@update
def accounts() -> Async[AccountsResult]:
    result: CanisterResult[list[Account]] = yield canister2.accounts()

    return {
        'ok': result.ok,
        'err': result.err
    }

@update
def trap() -> Async[TrapResult]:
    result: CanisterResult[str] = yield canister2.trap()

    return {
        'ok': result.ok,
        'err': result.err
    }

@update
def send_notification() -> Async[NotifyResult]:
    result: CanisterResult[None] = yield canister2.receive_notification('This is the notification').notify()

    return {
        'ok': result.ok,
        'err': result.err
    }
