# Management Canister

This chapter is a work in progress.

You can access the management canister like this:

```python
from kybra import Async, blob, update, Variant
from kybra.canisters.management import management_canister

class RandomBytesResult(Variant):
    ok: blob
    err: str

@update
def random_bytes() -> Async[RandomBytesResult]:
    return yield management_canister.raw_rand()
```

See the [management canister types](https://github.com/demergent-labs/kybra/blob/main/kybra/canisters/management/__init__.py) for all methods and their parameter and return types.
