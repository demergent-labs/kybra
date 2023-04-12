# http_request

This section is a work in progress.

Examples:

-   [ethereum_json_rpc](https://github.com/demergent-labs/kybra/tree/main/examples/ethereum_json_rpc)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)

```python
from kybra import Async, CallResult, match, ic, query, update
from kybra.canisters.management import (
    HttpResponse,
    HttpTransformArgs,
    management_canister,
)


@update
def xkcd() -> Async[HttpResponse]:
    max_response_bytes = 1_000

    # TODO this is just a heuristic for cost, might change when the feature is officially released: https://forum.dfinity.org/t/enable-canisters-to-make-http-s-requests/9670/130
    cycle_cost_base = 400_000_000
    cycle_cost_per_byte = 300_000  # TODO not sure on this exact cost
    cycle_cost_total = cycle_cost_base + cycle_cost_per_byte * max_response_bytes

    http_result: CallResult[HttpResponse] = yield management_canister.http_request(
        {
            "url": "https://xkcd.com/642/info.0.json",
            "max_response_bytes": max_response_bytes,
            "method": {"get": None},
            "headers": [],
            "body": None,
            "transform": {"function": (ic.id(), "xkcd_transform"), "context": bytes()},
        }
    ).with_cycles(cycle_cost_total)

    return match(http_result, {"Ok": lambda ok: ok, "Err": lambda err: ic.trap(err)})


@query
def xkcd_transform(args: HttpTransformArgs) -> HttpResponse:
    return {**args["response"], "headers": []}
```
