# http_request

This section is a work in progress.

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
