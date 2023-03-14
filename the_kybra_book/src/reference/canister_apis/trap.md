# trap

This section is a work in progress.

Examples:

-   [cross_canister_calls](https://github.com/demergent-labs/kybra/tree/main/examples/cross_canister_calls)
-   [ethereum_json_rpc](https://github.com/demergent-labs/kybra/tree/main/examples/ethereum_json_rpc)
-   [http_counter](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/http_counter)
-   [ic_api](https://github.com/demergent-labs/kybra/tree/main/examples/ic_api)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)

```python
from kybra import ic, query


# traps with a message, stopping execution and discarding all state within the call
@query
def trap(message: str) -> bool:
    ic.trap(message)

    return True
```
