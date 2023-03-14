# principal

This section is a work in progress.

The Kybra type `Principal` corresponds to the [Candid type principal](https://internetcomputer.org/docs/current/references/candid-ref#type-principal) and will become an [ic-py Principal](https://github.com/rocklabs-io/ic-py) at runtime.

Python:

```python
from kybra import ic, Principal, query

@query
def get_principal() -> Principal:
    return Principal.from_str('rrkah-fqaaa-aaaaa-aaaaq-cai')

@query
def print_principal(principal: Principal) -> Principal:
    ic.print(type(principal))
    return principal
```

Candid:

```python
service: {
    "get_principal": () -> (principal) query;
    "print_principal": (principal) -> (principal) query;
}
```
