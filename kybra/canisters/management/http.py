from kybra import blob, Func, nat, nat64, opt, Query, Record, Variant
from typing import TypeAlias


class HttpMethod(Variant, total=False):
    get: None
    head: None
    post: None


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


HttpTransformFunc: TypeAlias = Func(Query[[HttpTransformArgs], HttpResponse])


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
