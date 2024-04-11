# HTTP

This chapter is a work in progress.

## Incoming HTTP requests

Examples:

-   [http_counter](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/http_counter)

```python
from kybra import blob, Func, nat16, Opt, query, Query, Record, Tuple, Variant, Vec


class HttpRequest(Record):
    method: str
    url: str
    headers: Vec["Header"]
    body: blob


class HttpResponse(Record):
    status_code: nat16
    headers: Vec["Header"]
    body: blob
    streaming_strategy: Opt["StreamingStrategy"]
    upgrade: Opt[bool]


Header = Tuple[str, str]


class StreamingStrategy(Variant):
    Callback: "CallbackStrategy"


class CallbackStrategy(Record):
    callback: "Callback"
    token: "Token"


Callback = Func(Query[["Token"], "StreamingCallbackHttpResponse"])


class StreamingCallbackHttpResponse(Record):
    body: blob
    token: Opt["Token"]


class Token(Record):
    arbitrary_data: str


@query
def http_request(req: HttpRequest) -> HttpResponse:
    return {
        "status_code": 200,
        "headers": [],
        "body": bytes(),
        "streaming_strategy": None,
        "upgrade": False,
    }
```

## Outgoing HTTP requests

Examples:

-   [ethereum_json_rpc](https://github.com/demergent-labs/kybra/tree/main/examples/ethereum_json_rpc)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)

```python
from kybra import (
    Alias,
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

JSON = Alias[str]


stable_storage = StableBTreeMap[str, str](
    memory_id=0, max_key_size=20, max_value_size=1_000
)


@init
def init_(ethereum_url: str) -> void:
    stable_storage.insert("ethereum_url", ethereum_url)


@update
def eth_get_balance(ethereum_address: str) -> Async[JSON]:
    http_result: CallResult[HttpResponse] = yield management_canister.http_request(
        {
            "url": stable_storage.get("ethereum_url") or "",
            "max_response_bytes": 2_000,
            "method": {"post": None},
            "headers": [],
            "body": f'{{"jsonrpc":"2.0","method":"eth_getBalance","params":["{ethereum_address}","earliest"],"id":1}}'.encode(
                "utf-8"
            ),
            "transform": {"function": (ic.id(), "eth_transform"), "context": bytes()},
        }
    ).with_cycles(50_000_000)

    return match(
        http_result,
        {"Ok": lambda ok: ok["body"].decode("utf-8"), "Err": lambda err: ic.trap(err)},
    )


@update
def eth_get_block_by_number(number: nat32) -> Async[JSON]:
    http_result: CallResult[HttpResponse] = yield management_canister.http_request(
        {
            "url": stable_storage.get("ethereum_url") or "",
            "max_response_bytes": 2_000,
            "method": {"post": None},
            "headers": [],
            "body": f'{{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["{hex(number)}", false],"id":1}}'.encode(
                "utf-8"
            ),
            "transform": {"function": (ic.id(), "eth_transform"), "context": bytes()},
        }
    ).with_cycles(50_000_000)

    return match(
        http_result,
        {"Ok": lambda ok: ok["body"].decode("utf-8"), "Err": lambda err: ic.trap(err)},
    )


@query
def eth_transform(args: HttpTransformArgs) -> HttpResponse:
    http_response = args["response"]

    http_response["headers"] = []

    return http_response
```
