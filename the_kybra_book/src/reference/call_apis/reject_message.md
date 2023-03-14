# reject message

This section is a work in progress.

Examples:

-   [rejections](https://github.com/demergent-labs/kybra/tree/main/examples/rejections)

```python
from kybra import Async, ic, update
from src.some_service.types import some_service


@update
def get_rejection_message(message: str) -> Async[str]:
    yield some_service.reject(message)
    return ic.reject_message()
```
