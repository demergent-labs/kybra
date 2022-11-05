from kybra import Async, CanisterResult, nat64, NotifyResult, opt, Principal, update, Variant
from src.canister2.types import Account, AccountArgs, Canister2

class TransferResult(Variant, total=False):
    ok: nat64
    err: str

class BalanceResult(Variant, total=False):
    ok: nat64
    err: str

class AccountResult(Variant, total=False):
    ok: opt[Account]
    err: str

class AccountsResult(Variant, total=False):
    ok: list[Account]
    err: str

class TrapResult(Variant, total=False):
    ok: str
    err: str

canister2 = Canister2(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))

@update
def transfer(
    from_: str,
    to: str,
    amount: nat64
) -> Async[TransferResult]:
    result: CanisterResult[nat64] = yield canister2.transfer(from_, to, amount)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }

@update
def balance(id: str) -> Async[BalanceResult]:
    result: CanisterResult[nat64] = yield canister2.balance(id)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }

@update
def account(args: AccountArgs) -> Async[AccountResult]:
    result: CanisterResult[opt[Account]] = yield canister2.account(args)

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }

@update
def accounts() -> Async[AccountsResult]:
    result: CanisterResult[list[Account]] = yield canister2.accounts()

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }

@update
def trap() -> Async[TrapResult]:
    result: CanisterResult[str] = yield canister2.trap()

    if result.err is not None:
        return {
            'err': result.err
        }

    return {
        'ok': result.ok
    }

# TODO we need to figure out some kind of type guard/assertion thing for variants
@update
def send_notification() -> NotifyResult:
    result = canister2.receive_notification('This is the notification').notify()

    if 'err' in result:
        return {
            'err': result['err']
        }

    return {
        'ok': result['ok']
    }
