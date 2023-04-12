# service

This section is a work in progress.

Python classes that inherit from the Kybra type `Service` correspond to the [Candid service type](https://internetcomputer.org/docs/current/references/candid-ref#type-service-).

Python:

```python
class SomeService(Service):
    @service_query
    def query1(self) -> bool:
        ...

    @service_update
    def update1(self) -> str:
        ...


some_service = SomeService(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))
```

Candid:

```
service { query1 : () -> (bool) query; update1 : () -> (text) }
```
