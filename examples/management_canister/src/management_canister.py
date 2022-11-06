from kybra import CanisterResult, query, update, Async, blob
from kybra.canisters.management import management_canister

@query
def test() -> str:
    return 'hello'

@update
def raw_rand() -> Async[blob]:
    result: CanisterResult[blob] = yield management_canister.raw_rand()

    return result.ok
