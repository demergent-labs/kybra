from kybra import Async, CanisterResult, ic, init, nat32, query, update
from kybra.canisters.management import management_canister
from kybra.canisters.management.http import HttpResponse, HttpTransformArgs
from typing import TypeAlias, TypedDict

JSON: TypeAlias = str


class StableStorage(TypedDict):
    ethereum_url: str


stable_storage: StableStorage = ic.stable_storage()


@init
def init_(ethereum_url: str):
    stable_storage['ethereum_url'] = ethereum_url


@update
def eth_get_balance(ethereum_address: str) -> Async[JSON]:
    max_response_bytes = 1_000

    # TODO this is just a heuristic for cost, might change when the feature is officially released: https://forum.dfinity.org/t/enable-canisters-to-make-http-s-requests/9670/130
    cycle_cost_base = 400_000_000
    cycle_cost_per_byte = 300_000  # TODO not sure on this exact cost
    cycle_cost_total = cycle_cost_base + cycle_cost_per_byte * max_response_bytes

    http_result: CanisterResult[HttpResponse] = yield management_canister.http_request({
        'url': stable_storage['ethereum_url'],
        'max_response_bytes': max_response_bytes,
        'method': {'post': None},
        'headers': [],
        'body': '{{"jsonrpc":"2.0","method":"eth_getBalance","params":["{address}","earliest"],"id":1}}'.format(address=ethereum_address).encode('utf-8'),
        'transform': {
            'function': ((ic.id()), 'eth_transform'),
            'context': bytes()
        },
    }).with_cycles(cycle_cost_total)

    if http_result.err is not None:
        if http_result.err:
            ic.trap(http_result.err)
        ic.trap('http_result had an error')

    return http_result.ok['body'].decode('utf-8', 'strict')


@update
def eth_get_block_by_number(number: nat32) -> Async[JSON]:
    max_response_bytes = 2_000

    # TODO this is just a heuristic for cost, might change when the feature is officially released: https://forum.dfinity.org/t/enable-canisters-to-make-http-s-requests/9670/130
    cycle_cost_base = 400_000_000
    cycle_cost_per_byte = 300_000  # TODO not sure on this exact cost
    cycle_cost_total = cycle_cost_base + cycle_cost_per_byte * max_response_bytes

    http_result: CanisterResult[HttpResponse] = yield management_canister.http_request({
        'url': stable_storage['ethereum_url'],
        'max_response_bytes': max_response_bytes,
        'method': {'post': None},
        'headers': [],
        'body': '{{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["{block_number}", false],"id":1}}'.format(block_number=hex(number)).encode('utf-8'),
        'transform': {
            'function': ((ic.id()), 'eth_transform'),
            'context': bytes()
        },
    }).with_cycles(cycle_cost_total)

    if http_result.err is not None:
        if http_result.err:
            ic.trap(http_result.err)
        ic.trap('http_result had an error')

    return http_result.ok['body'].decode('utf-8', 'strict')


@query
def eth_transform(args: HttpTransformArgs) -> HttpResponse:
    return {
        **args['response'],
        'headers': []
    }
