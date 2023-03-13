# candid decode

This section is a work in progress.

Examples:

-   [call_raw](https://github.com/demergent-labs/kybra/tree/main/examples/call_raw)
-   [candid_encoding](https://github.com/demergent-labs/kybra/tree/main/examples/candid_encoding)

```python
from kybra import blob, ic, query


# decodes Candid bytes to a Candid string
@query
def candid_decode(candid_encoded: blob) -> str:
    return ic.candid_decode(candid_encoded)
```
