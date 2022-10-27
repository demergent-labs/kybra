from kybra import nat64, query, update
from typing import TypedDict

class Account(TypedDict):
    address: str
    balance: nat64

class State(TypedDict):
    accounts: dict[str, Account]
    name: str
    ticker: str
    total_supply: nat64

state: State = {
    'accounts': {},
    'name': '',
    'ticker': '',
    'total_supply': 0
}

@update
def initialize_supply(
    name: str,
    original_address: str,
    ticker: str,
    total_supply: nat64
) -> bool:
    global state

    state = {
        'accounts': {
            original_address: {
                'address': original_address,
                'balance': total_supply
            }
        },
        'name': name,
        'ticker': ticker,
        'total_supply': total_supply
    }

    return True

@update
def transfer(
    from_address: str,
    to_address: str,
    amount: nat64
) -> bool:
    if state['accounts'].get(to_address, None) is None:
        state['accounts'][to_address] = {
            'address': to_address,
            'balance': 0
        }

    state['accounts'][from_address]['balance'] -= amount
    state['accounts'][to_address]['balance'] += amount

    return True

@query
def balance(address: str) -> nat64:
    return state['accounts'].get(address, {'address': '-1', 'balance': 0})['balance']

@query
def ticker() -> str:
    return state['ticker']

@query
def name() -> str:
    return state['name']

@query
def total_supply() -> nat64:
    return state['total_supply']
