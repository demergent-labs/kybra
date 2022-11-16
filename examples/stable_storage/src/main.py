from kybra import ic, pre_upgrade, init, post_upgrade
from typing import TypedDict

class StableStorage(TypedDict):
    hello: str
    there: int

stable_storage: StableStorage = ic.stable_storage()

@init
def init_():
    ic.print('init')
    ic.print(stable_storage)

@pre_upgrade
def pre_upgrade_():
    ic.print('pre_upgrade')
    stable_storage['hello'] = 'world'
    stable_storage['there'] = 5

@post_upgrade
def post_upgrade_():
    ic.print('post_upgrade')
    ic.print(stable_storage['hello'])
    ic.print(stable_storage['there'])
