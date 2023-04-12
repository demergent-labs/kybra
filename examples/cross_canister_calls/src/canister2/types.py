from kybra import nat64, Opt, Record, Service, service_query, service_update, Vec, void
from typing import TypedDict


class Account(Record):
    id: str
    balance: nat64


class AccountArgs(Record):
    id: str


class Canister2(Service):
    @service_update
    def transfer(self, from_: str, to: str, amount: nat64) -> nat64:
        ...

    @service_query
    def balance(self, id: str) -> nat64:
        ...

    @service_query
    def account(self, account_args: AccountArgs) -> Opt[Account]:
        ...

    @service_query
    def accounts(self) -> Vec[Account]:
        ...

    @service_query
    def trap(self) -> str:
        ...

    @service_update
    def receive_notification(self, message: str) -> void:
        ...


class State(TypedDict):
    accounts: dict[str, "Account"]
    notification: str
