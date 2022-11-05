from kybra import Canister, method, nat64, opt, Record
from typing import TypedDict

class Account(Record):
    id: str
    balance: nat64

class AccountArgs(Record):
    id: str

class Canister2(Canister):
    @method
    def transfer(self, from_: str, to: str, amount: nat64) -> nat64: ...

    @method
    def balance(self, id: str) -> nat64: ...

    @method
    def account(self, account_args: AccountArgs) -> opt[Account]: ...

    @method
    def accounts(self) -> list[Account]: ...

    @method
    def trap(self) -> str: ...

    @method
    def receive_notification(self, message: str) -> None: ...

class State(TypedDict):
    accounts: dict[str, "Account"]
    notification: str
