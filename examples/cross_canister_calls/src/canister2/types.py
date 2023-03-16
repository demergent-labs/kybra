from kybra import Canister, nat64, opt, query, Record, update, void
from typing import TypedDict


class Account(Record):
    id: str
    balance: nat64


class AccountArgs(Record):
    id: str


class Canister2(Canister):
    @update
    def transfer(self, from_: str, to: str, amount: nat64) -> nat64: ...

    @query
    def balance(self, id: str) -> nat64: ...

    @query
    def account(self, account_args: AccountArgs) -> opt[Account]: ...

    @query
    def accounts(self) -> list[Account]: ...

    @query
    def trap(self) -> str: ...

    @update
    def receive_notification(self, message: str) -> void: ...


class State(TypedDict):
    accounts: dict[str, "Account"]
    notification: str
