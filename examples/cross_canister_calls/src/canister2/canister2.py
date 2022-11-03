from kybra import ic, nat64, opt, query, update
from .types import Account, AccountArgs, State

state: State = {
    'accounts': {
        '0': {
            'id': '0',
            'balance': 100
        }
    },
    'notification': ''
}

@update
def transfer(
    from_: str,
    to: str,
    amount: nat64
) -> nat64:
    # TODO check and make sure this all works well
    from_balance = state['accounts'][from_].get('balance', 0)

    if from_balance < amount:
        return 0

    to_balance = state['accounts'][to]['balance']

    if  to_balance is None:
        state['accounts'][to] = {
            'id': to,
            'balance': 0
        }

    state['accounts'][from_]['balance'] -= amount
    state['accounts'][to]['balance'] += amount

    return amount

@query
def balance(id: str) -> nat64:
    return state['accounts'][id].get('balance', 0) # TODO test none stuff

@query
def account(account_args: AccountArgs) -> opt[Account]:
    return state['accounts'].get(account_args['id'], None)

@query
def accounts() -> list[Account]:
    return list(state['accounts'].values())

@query
def trap() -> str:
    ic.trap('hahahaha')
    return 'You will never get here'

@update
def receive_notification(message: str):
    state['notification'] = message

@query
def get_notification() -> str:
    return state['notification']
