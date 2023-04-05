# reject code

This section is a work in progress.

Examples:

-   [rejections](https://github.com/demergent-labs/kybra/tree/main/examples/rejections)

```python
from kybra import Async, ic, Principal, RejectionCode, Service, service_update, update, void


class Nonexistent(Service):
    @service_update
    def method(self) -> void:
        ...


nonexistent_canister = Nonexistent(Principal.from_str("rkp4c-7iaaa-aaaaa-aaaca-cai"))


@update
def get_rejection_code_destination_invalid() -> Async[RejectionCode]:
    yield nonexistent_canister.method()
    return ic.reject_code()
```
