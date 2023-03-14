# raw_rand

This section is a work in progress.

Examples:

-   [generators](https://github.com/demergent-labs/kybra/tree/main/examples/generators)
-   [heartbeat](https://github.com/demergent-labs/kybra/tree/main/examples/heartbeat)
-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)
-   [timers](https://github.com/demergent-labs/kybra/tree/main/examples/timers)

```python
from kybra import Async, blob, CanisterResult, update
from kybra.canisters.management import management_canister


@update
def get_randomness_directly() -> Async[blob]:
    randomness_result: CanisterResult[blob] = yield management_canister.raw_rand()

    if randomness_result.err is not None:
        return bytes()

    return randomness_result.ok
```
