from kybra import blob, Func, nat, nat64, null, Opt, Query, Record, Variant, Vec


class HttpMethod(Variant, total=False):
    get: null
    head: null
    post: null


class HttpHeader(Record):
    name: str
    value: str


class HttpResponse(Record):
    status: nat
    headers: Vec[HttpHeader]
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
    max_response_bytes: Opt[nat64]
    method: HttpMethod
    headers: Vec[HttpHeader]
    body: Opt[blob]
    transform: Opt[HttpTransform]
