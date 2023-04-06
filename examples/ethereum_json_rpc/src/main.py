from kybra import (
    alias,
    Async,
    CallResult,
    ic,
    match,
    init,
    nat32,
    query,
    StableBTreeMap,
    update,
    void,
)
from kybra.canisters.management import (
    HttpResponse,
    HttpTransformArgs,
    management_canister,
)

JSON = alias[str]


stable_storage = StableBTreeMap[str, str](
    memory_id=0, max_key_size=20, max_value_size=1_000
)


@init
def init_(ethereum_url: str) -> void:
    stable_storage.insert("ethereum_url", ethereum_url)


@update
def eth_get_balance(ethereum_address: str) -> Async[JSON]:
    max_response_bytes = 200

    # TODO this is just a heuristic for cost, might change when the feature is officially released: https://forum.dfinity.org/t/enable-canisters-to-make-http-s-requests/9670/130
    cycle_cost_base = 400_000_000
    cycle_cost_per_byte = 300_000  # TODO not sure on this exact cost
    cycle_cost_total = cycle_cost_base + cycle_cost_per_byte * max_response_bytes

    http_result: CallResult[HttpResponse] = yield management_canister.http_request(
        {
            "url": stable_storage.get("ethereum_url") or "",
            "max_response_bytes": max_response_bytes,
            "method": {"post": None},
            "headers": [],
            "body": f'{{"jsonrpc":"2.0","method":"eth_getBalance","params":["{ethereum_address}","earliest"],"id":1}}'.encode(
                "utf-8"
            ),
            "transform": {"function": (ic.id(), "eth_transform"), "context": bytes()},
        }
    ).with_cycles(cycle_cost_total)

    return match(
        http_result,
        {"Ok": lambda ok: ok["body"].decode("utf-8"), "Err": lambda err: ic.trap(err)},
    )


@update
def eth_get_block_by_number(number: nat32) -> Async[JSON]:
    max_response_bytes = 2_000

    # TODO this is just a heuristic for cost, might change when the feature is officially released: https://forum.dfinity.org/t/enable-canisters-to-make-http-s-requests/9670/130
    cycle_cost_base = 400_000_000
    cycle_cost_per_byte = 300_000  # TODO not sure on this exact cost
    cycle_cost_total = cycle_cost_base + cycle_cost_per_byte * max_response_bytes

    http_result: CallResult[HttpResponse] = yield management_canister.http_request(
        {
            "url": stable_storage.get("ethereum_url") or "",
            "max_response_bytes": max_response_bytes,
            "method": {"post": None},
            "headers": [],
            "body": f'{{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["{hex(number)}", false],"id":1}}'.encode(
                "utf-8"
            ),
            "transform": {"function": (ic.id(), "eth_transform"), "context": bytes()},
        }
    ).with_cycles(cycle_cost_total)

    return match(
        http_result,
        {"Ok": lambda ok: ok["body"].decode("utf-8"), "Err": lambda err: ic.trap(err)},
    )


@query
def eth_transform(args: HttpTransformArgs) -> HttpResponse:
    return {**args["response"], "headers": []}
