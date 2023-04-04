# Management Canister

This chapter is a work in progress.

You can access the management canister like this:

```python
from kybra import Async, blob, CallResult, update, Variant
from kybra.canisters.management import management_canister

class RandomBytesResult(Variant, total=False):
    ok: blob
    err: str

@update
def random_bytes() -> Async[RandomBytesResult]:
    call_result: CallResult[blob] = yield management_canister.raw_rand()

    if call_result.err is not None:
        return {
            'err': call_result.err
        }

    return {
        'ok': call_result.ok
    }
```

See the [management canister types](https://github.com/demergent-labs/kybra/blob/main/kybra/canisters/management/__init__.py) for all methods and their parameter and return types.
