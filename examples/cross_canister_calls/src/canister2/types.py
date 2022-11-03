from kybra import call, Canister, nat64, opt, Record
from typing import TypedDict

class Account(Record):
    id: str
    balance: nat64

class AccountArgs(Record):
    id: str

class Canister2(Canister):
    @call
    def transfer(self, from_: str, to: str, amount: nat64) -> nat64: ...

    @call
    def balance(self, id: str) -> nat64: ...

    @call
    def account(self, account_args: AccountArgs) -> opt[Account]: ...

    @call
    def accounts(self) -> list[Account]: ...

    @call
    def trap(self) -> str: ...

    @call
    def receive_notification(self, message: str): ...

class State(TypedDict):
    accounts: dict[str, "Account"]
    notification: str
