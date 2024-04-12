from kybra import (
    Alias,
    blob,
    Func,
    ic,
    nat,
    nat16,
    Opt,
    Query,
    query,
    Record,
    StableBTreeMap,
    Tuple,
    update,
    Variant,
    Vec,
)


class Token(Record):
    arbitrary_data: str


class StreamingCallbackHttpResponse(Record):
    body: blob
    token: Opt[Token]


Callback = Func(Query[[Token], StreamingCallbackHttpResponse])


class CallbackStrategy(Record):
    callback: Callback
    token: Token


class StreamingStrategy(Variant, total=False):
    Callback: CallbackStrategy


HeaderField = Alias[Tuple[str, str]]


class HttpResponse(Record):
    status_code: nat16
    headers: Vec[HeaderField]
    body: blob
    streaming_strategy: Opt[StreamingStrategy]
    upgrade: Opt[bool]


class HttpRequest(Record):
    method: str
    url: str
    headers: Vec[HeaderField]
    body: blob


stable_storage = StableBTreeMap[str, nat](
    memory_id=0, max_key_size=15, max_value_size=1_000
)

stable_storage.insert("counter", 0)


def isGzip(x: HeaderField) -> bool:
    return x[0].lower() == "accept-encoding" and "gzip" in x[1].lower()


@query
def http_request(req: HttpRequest) -> HttpResponse:
    ic.print("Hello from http_request")

    if req["method"] == "GET":
        if next(filter(isGzip, req["headers"]), None) is None:
            if req["url"] == "/stream":
                return {
                    "status_code": 200,
                    "headers": [("content-type", "text/plain")],
                    "body": "Counter".encode("utf-8"),
                    "streaming_strategy": {
                        "Callback": {
                            "callback": (ic.id(), "http_streaming"),
                            "token": {"arbitrary_data": "start"},
                        }
                    },
                    "upgrade": False,
                }
            return {
                "status_code": 200,
                "headers": [("content-type", "text/plain")],
                "body": f"Counter is {stable_storage.get('counter')}\n{req['url']}".encode(
                    "utf-8"
                ),
                "streaming_strategy": None,
                "upgrade": None,
            }
        return {
            "status_code": 200,
            "headers": [("content-type", "text/plain"), ("content-encoding", "gzip")],
            "body": bytes(
                [
                    31,
                    139,
                    8,
                    0,
                    152,
                    2,
                    27,
                    98,
                    0,
                    3,
                    43,
                    44,
                    77,
                    45,
                    170,
                    228,
                    2,
                    0,
                    214,
                    128,
                    43,
                    5,
                    6,
                    0,
                    0,
                    0,
                ]
            ),
            "streaming_strategy": None,
            "upgrade": None,
        }

    if req["method"] == "POST":
        return {
            "status_code": 204,
            "headers": [],
            "body": "".encode("utf-8"),
            "streaming_strategy": None,
            "upgrade": True,
        }

    return {
        "status_code": 400,
        "headers": [],
        "body": "Invalid request".encode("utf-8"),
        "streaming_strategy": None,
        "upgrade": None,
    }


@update
def http_request_update(req: HttpRequest) -> HttpResponse:
    ic.print("Hello from update")
    global stable_storage

    if req["method"] == "POST":
        counter = stable_storage.get("counter") or 0
        stable_storage.insert("counter", counter + 1)

        if next(filter(isGzip, req["headers"]), None) is None:
            return {
                "status_code": 201,
                "headers": [("content-type", "text/plain")],
                "body": f"Counter updated to {stable_storage.get('counter')}".encode(
                    "utf-8"
                ),
                "streaming_strategy": None,
                "upgrade": None,
            }
        return {
            "status_code": 201,
            "headers": [("content-type", "text/plain"), ("content-encoding", "gzip")],
            "body": bytes(
                [
                    31,
                    139,
                    8,
                    0,
                    55,
                    2,
                    27,
                    98,
                    0,
                    3,
                    43,
                    45,
                    72,
                    73,
                    44,
                    73,
                    229,
                    2,
                    0,
                    168,
                    218,
                    145,
                    108,
                    7,
                    0,
                    0,
                    0,
                ]
            ),
            "streaming_strategy": None,
            "upgrade": None,
        }

    return {
        "status_code": 400,
        "headers": [],
        "body": "Invalid request".encode("utf-8"),
        "streaming_strategy": None,
        "upgrade": None,
    }


@query
def http_streaming(token: Token) -> StreamingCallbackHttpResponse:
    ic.print("Hello from http_streaming")
    # match token['arbitrary_data']:
    #     case 'start':
    #         return {
    #             'body': ' is '.encode('utf-8'),
    #             'token': {'arbitrary_data': 'next'}
    #         }
    #     case 'next':
    #         return {
    #             'body': f"{stable_storage['counter']}".encode('utf-8'),
    #             'token': {'arbitrary_data': 'last'}
    #         }
    #     case 'last':
    #         return {
    #             'body': ' streaming\n'.encode('utf-8'),
    #             'token': None
    #         }
    #     case _:
    #         ic.trap('unreachable')
    if token["arbitrary_data"] == "start":
        return {"body": " is ".encode("utf-8"), "token": {"arbitrary_data": "next"}}
    if token["arbitrary_data"] == "next":
        return {
            "body": f"{stable_storage.get('counter')}".encode("utf-8"),
            "token": {"arbitrary_data": "last"},
        }
    if token["arbitrary_data"] == "last":
        return {"body": " streaming\n".encode("utf-8"), "token": None}
    ic.trap("unreachable")
