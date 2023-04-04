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
    canister_result: CallResult[blob] = yield management_canister.raw_rand()

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': canister_result.ok
    }
```

See the [management canister types](https://github.com/demergent-labs/kybra/blob/main/kybra/canisters/management/__init__.py) for all methods and their parameter and return types.
