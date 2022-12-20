from kybra import Async, blob, CanisterResult, ic, Principal, query, update, manual
from kybra.canisters.management import management_canister
from kybra.canisters.management.http import HttpResponse, HttpTransformArgs


@update
def xkcd() -> Async[HttpResponse]:
    max_response_bytes = 1_000

    # TODO this is just a heuristic for cost, might change when the feature is officially released: https://forum.dfinity.org/t/enable-canisters-to-make-http-s-requests/9670/130
    cycle_cost_base = 400_000_000
    cycle_cost_per_byte = 300_000  # TODO not sure on this exact cost
    cycle_cost_total = cycle_cost_base + cycle_cost_per_byte * max_response_bytes

    http_result: CanisterResult[HttpResponse] = yield management_canister.http_request({
        'url': 'https://xkcd.com/642/info.0.json',
        'max_response_bytes': max_response_bytes,
        'method': {'get': None},
        'headers': [],
        'body': None,
        'transform': {
            'function': (ic.id(), 'xkcd_transform'),
            'context': bytes()
        },
    }).with_cycles(cycle_cost_total)

    if http_result.err is not None:
        if http_result.err:
            ic.trap(http_result.err)
        ic.trap('http_result had an error')

    return http_result.ok


@update
def xkcd_raw() -> Async[manual[HttpResponse]]:
    max_response_bytes = 1_000

    # TODO this is just a heuristic for cost, might change when the feature is officially released: https://forum.dfinity.org/t/enable-canisters-to-make-http-s-requests/9670/130
    cycle_cost_base = 400_000_000
    cycle_cost_per_byte = 300_000  # TODO not sure on this exact cost
    cycle_cost_total = cycle_cost_base + cycle_cost_per_byte * max_response_bytes

    http_result: CanisterResult[blob] = yield ic.call_raw(
        Principal.from_str('aaaaa-aa'),
        'http_request',
        ic.candid_encode(
            f"""
                (
                    record {{
                        url = "https://xkcd.com/642/info.0.json";
                        max_response_bytes = {max_response_bytes} : nat64;
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
        cycle_cost_total
    )

    if http_result.err is not None:
        if http_result.err:
            ic.trap(http_result.err)
        ic.trap('http_result had an error')

    ic.reply_raw(http_result.ok)


@query
def xkcd_transform(args: HttpTransformArgs) -> HttpResponse:
    return {
        **args['response'],
        'headers': []
    }
