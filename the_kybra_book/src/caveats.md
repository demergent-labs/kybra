# Caveats

## Unknown security vulnerabilities

Kybra is a beta project using a new Python interpreter. See the [disclaimer](./kybra.md#disclaimer) for more information.

## No C extensions

Any PyPI packages or other Python code that relies on C extensions will not currently work. It is a major goal for us to support C extensions in the future.

## Performance

Kybra is probably ~7-20x less performant than what you would expect from [CPython](https://github.com/python/cpython). We hope to eventually use `CPython` as Kybra's underlying Python interpreter.

## Do not use dictionary unpacking

A bug in the [RustPython](https://github.com/RustPython/RustPython) interpreter means that dictionary unpacking should not be used until [this issue](https://github.com/RustPython/RustPython/issues/4932) is addressed.

## Reserved memory ids

`memory ids` `0`, `1`, `2`, and `254` are currently reserved for `StableBTreeMap`.

## print does not work

You should use `ic.print` instead of `print`.

## Kybra types

### Imports

Make sure to use the `from kybra` syntax when importing types from the `kybra` module, and to not rename types with `as`:

Correct:

```python
from kybra import Record

class MyRecord(Record):
    prop1: str
    prop2: int
```

Incorrect:

```python
import kybra

class MyRecord(kybra.Record):
    prop1: str
    prop2: int
```

Incorrect:

```python
from kybra import Record as RenamedRecord

class MyRecord(RenamedRecord):
    prop1: str
    prop2: int
```

We wish to improve this situation in the future to handle the many various ways of importing.

### Treatment as keywords

You should treat Kybra types essentially as keywords, not creating types of the same name elsewhere in your codebase or in other libraries. Any types exported from [this file](https://github.com/demergent-labs/kybra/blob/main/kybra/__init__.py) should be treated thusly.
