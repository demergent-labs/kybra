# update

This section is a work in progress.

```python
from kybra import query, update, void

message = ""


@query
def get_message() -> str:
    return message


@update
def set_message(new_message: str) -> void:
    global message
    message = new_message
```
