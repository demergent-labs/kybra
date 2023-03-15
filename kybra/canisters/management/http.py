from kybra import blob, Func, nat, nat64, null, opt, Query, Record, Variant


class HttpMethod(Variant, total=False):
    get: null
    head: null
    post: null


class HttpHeader(Record):
    name: str
    value: str


class HttpResponse(Record):
    status: nat
    headers: list[HttpHeader]
    body: blob


class HttpTransformArgs(Record):
    response: HttpResponse
    context: blob


HttpTransformFunc = Func(Query[[HttpTransformArgs], HttpResponse])


class HttpTransform(Record):
    function: HttpTransformFunc
    context: blob


class HttpRequestArgs(Record):
    url: str
    max_response_bytes: opt[nat64]
    method: HttpMethod
    headers: list[HttpHeader]
    body: opt[blob]
    transform: opt[HttpTransform]
