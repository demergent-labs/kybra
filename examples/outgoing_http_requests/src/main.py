from kybra import (
    Async,
    blob,
    CallResult,
    ic,
    match,
    Principal,
    query,
    update,
    Manual,
)
from kybra.canisters.management import (
    HttpResponse,
    HttpTransformArgs,
    management_canister,
)


@update
def xkcd() -> Async[HttpResponse]:
    http_result: CallResult[HttpResponse] = yield management_canister.http_request(
        {
            "url": "https://xkcd.com/642/info.0.json",
            "max_response_bytes": 2_000,
            "method": {"get": None},
            "headers": [],
            "body": None,
            "transform": {"function": (ic.id(), "xkcd_transform"), "context": bytes()},
        }
    ).with_cycles(50_000_000)

    return match(http_result, {"Ok": lambda ok: ok, "Err": lambda err: ic.trap(err)})


@update
def xkcd_raw() -> Async[Manual[HttpResponse]]:
    http_result: CallResult[blob] = yield ic.call_raw(
        Principal.from_str("aaaaa-aa"),
        "http_request",
        ic.candid_encode(
            f"""
                (
                    record {{
                        url = "https://xkcd.com/642/info.0.json";
                        max_response_bytes = 2_000 : nat64;
                        method = variant {{ get }};
                        headers = vec {{}};
                        body = null;
                        transform = opt record {{
                            function = record {{ principal "{ic.id()}"; "xkcd_transform" }};
                            context = vec {{}}
                        }};
                    }}
                )
            """
        ),
        50_000_000,
    )

    match(
        http_result,
        {"Ok": lambda ok: ic.reply_raw(ok), "Err": lambda err: ic.trap(err)},
    )


@query
def xkcd_transform(args: HttpTransformArgs) -> HttpResponse:
    http_response = args["response"]

    http_response["headers"] = []

    return http_response
