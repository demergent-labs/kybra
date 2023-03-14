# canister id

This section is a work in progress.

Examples:

-   [ethereum_json_rpc](https://github.com/demergent-labs/kybra/tree/main/examples/ethereum_json_rpc)
-   [ic_api](https://github.com/demergent-labs/kybra/tree/main/examples/ic_api)
-   [http_counter](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/http_counter)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)
-   [whoami](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/whoami)

```python
from kybra import ic, Principal, query


# returns this canister's id
@query
def id() -> Principal:
    return ic.id()
```
