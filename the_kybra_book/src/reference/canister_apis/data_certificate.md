# data certificate

This section is a work in progress.

Examples:

-   [ic_api](https://github.com/demergent-labs/kybra/tree/main/examples/ic_api)

```python
from kybra import blob, ic, Opt, query


# When called from a query call, returns the data certificate authenticating certified_data set by this canister. Returns None if not called from a query call.
@query
def data_certificate() -> Opt[blob]:
    return ic.data_certificate()
```
