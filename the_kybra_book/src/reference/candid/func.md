# func

This section is a work in progress.

WARNING: `Func` currently [causes type errors in VS Code](https://github.com/demergent-labs/kybra/issues/229). Get around this by ending your `Func` type declaration with the following comment: `# type: ignore`

The Kybra type `Func` corresponds to the [Candid type func](https://internetcomputer.org/docs/current/references/candid-ref#type-func---) and at runtime will become a Python tuple with two elements, the first being an [ic-py Principal](https://github.com/rocklabs-io/ic-py) and the second being a [Python str](https://docs.python.org/3/library/stdtypes.html#textseq). The `ic-py Principal` represents the `principal` of the canister/service where the function exists, and the `str` represents the function's name.

Note that an explicit `TypeAlias` must be used when defining a `func`.

Python:

```python
from kybra import Func, nat64, null, Principal, query, Query, Record, Update, Variant


class User(Record):
    id: str
    basic_func: "BasicFunc"
    complex_func: "ComplexFunc"


class Reaction(Variant, total=False):
    Good: null
    Bad: null
    BasicFunc: "BasicFunc"
    ComplexFunc: "ComplexFunc"


BasicFunc = Func(Query[[str], str])
ComplexFunc = Func(Update[[User, Reaction], nat64])


@query
def get_basic_func() -> BasicFunc:
    return (Principal.from_str("rrkah-fqaaa-aaaaa-aaaaq-cai"), "simple_function_name")


@query
def get_complex_func() -> ComplexFunc:
    return (Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"), "complex_function_name")
```

Candid:

```python
type User = record {
    "id": text;
    "basic_func": BasicFunc;
    "complex_func": ComplexFunc;
};
type Reaction = variant { "Good": null; "Bad": null; "BasicFunc": BasicFunc; "ComplexFunc": ComplexFunc };

type BasicFunc = func (text) -> (text) query;
type ComplexFunc = func (User, Reaction) -> (nat64);

service: () -> {
    "get_basic_func": () -> (BasicFunc) query;
    "get_complex_func": () -> (ComplexFunc) query;
}
```
