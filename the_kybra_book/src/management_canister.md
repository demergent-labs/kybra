# Management Canister

This chapter is a work in progress.

You can access the management canister like this:

```python
from kybra import Async, blob, CallResult, match, update, Variant
from kybra.canisters.management import management_canister


class RandomBytesResult(Variant, total=False):
    Ok: blob
    Err: str


@update
def random_bytes() -> Async[RandomBytesResult]:
    call_result: CallResult[blob] = yield management_canister.raw_rand()

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )
```

See the [management canister types](https://github.com/demergent-labs/kybra/blob/main/kybra/canisters/management/__init__.py) for all methods and their parameter and return types.
