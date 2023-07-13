# reject message

This section is a work in progress.

Examples:

-   [rejections](https://github.com/demergent-labs/kybra/tree/main/examples/rejections)

```python
from kybra import Async, ic, Principal, Service, service_update, update, void


class SomeService(Service):
    @service_update
    def reject(self, message: str) -> void:
        ...


some_service = SomeService(Principal.from_str("rkp4c-7iaaa-aaaaa-aaaca-cai"))


@update
def get_rejection_message(message: str) -> Async[str]:
    yield some_service.reject(message)
    return ic.reject_message()
```
