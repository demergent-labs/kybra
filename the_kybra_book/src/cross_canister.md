# Cross-canister

Examples:

-   [bitcoin](https://github.com/demergent-labs/kybra/tree/main/examples/bitcoin)
-   [composite_queries](https://github.com/demergent-labs/kybra/tree/main/examples/composite_queries)
-   [cross_canister_calls](https://github.com/demergent-labs/kybra/tree/main/examples/cross_canister_calls)
-   [cycles](https://github.com/demergent-labs/kybra/tree/main/examples/cycles)
-   [ethereum_json_rpc](https://github.com/demergent-labs/kybra/tree/main/examples/ethereum_json_rpc)
-   [func_types](https://github.com/demergent-labs/kybra/tree/main/examples/func_types)
-   [heartbeat](https://github.com/demergent-labs/kybra/tree/main/examples/heartbeat)
-   [generators](https://github.com/demergent-labs/kybra/tree/main/examples/generators)
-   [ledger_canister](https://github.com/demergent-labs/kybra/tree/main/examples/ledger_canister)
-   [management_canister](https://github.com/demergent-labs/kybra/tree/main/examples/management_canister)
-   [outgoing_http_requests](https://github.com/demergent-labs/kybra/tree/main/examples/outgoing_http_requests)
-   [threshold_ecdsa](https://github.com/demergent-labs/kybra/tree/main/examples/threshold_ecdsa)
-   [rejections](https://github.com/demergent-labs/kybra/tree/main/examples/rejections)
-   [timers](https://github.com/demergent-labs/kybra/tree/main/examples/timers)
-   [whoami](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/whoami)

Canisters are generally able to call the query or update methods of other canisters in any subnet. We refer to these types of calls as cross-canister calls.

A cross-canister call begins with a definition of the canister to be called, referred to as a service.

Imagine a simple service called `token_canister`:

```python
from kybra import ic, nat64, Principal, StableBTreeMap, update

accounts = StableBTreeMap[Principal, nat64](
    memory_id=0, max_key_size=38, max_value_size=15
)


@update
def transfer(to: Principal, amount: nat64) -> nat64:
    from_ = ic.caller()

    from_balance = accounts.get(from_) or 0
    to_balance = accounts.get(to) or 0

    accounts.insert(from_, from_balance - amount)
    accounts.insert(to, to_balance + amount)

    return amount
```

Here's how you would create its service definition:

```python
from kybra import nat64, Principal, Service, service_update


class TokenCanister(Service):
    @service_update
    def transfer(self, to: Principal, amount: nat64) -> nat64:
        ...
```

Once you have a service definition you can instantiate it with the service's `Principal` and then invoke its methods.

Here's how to instantiate `TokenCanister`:

```python
token_canister = TokenCanister(
    Principal.from_str('r7inp-6aaaa-aaaaa-aaabq-cai')
)
```

And here's a more complete example of a service called `payout_canister` that performs a cross-canister call to `token_canister`:

```python
from kybra import (
    Async,
    CallResult,
    match,
    nat64,
    Principal,
    Service,
    service_update,
    update,
    Variant,
)


class TokenCanister(Service):
    @service_update
    def transfer(self, to: Principal, amount: nat64) -> nat64:
        ...


token_canister = TokenCanister(Principal.from_str("r7inp-6aaaa-aaaaa-aaabq-cai"))


class PayoutResult(Variant, total=False):
    Ok: nat64
    Err: str


@update
def payout(to: Principal, amount: nat64) -> Async[PayoutResult]:
    result: CallResult[nat64] = yield token_canister.transfer(to, amount)

    return match(result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}})
```

Notice that the `token_canister.transfer` method, because it is a cross-canister method, returns a `CallResult`. All cross-canister calls return `CallResult`, which has an `Ok` or `Err` property depending on if the cross-canister call was successful or not.

The IC guarantees that cross-canister calls will return. This means that, generally speaking, you will always receive a `CallResult`. Kybra does not raise on cross-canister calls. Wrapping your cross-canister call in a `try...except` most likely won't do anything useful.

Let's add to our example code and explore adding some practical result-based error-handling to stop people from stealing tokens.

`token_canister`:

```python
from kybra import ic, nat64, Principal, StableBTreeMap, update, Variant

accounts = StableBTreeMap[Principal, nat64](
    memory_id=0, max_key_size=38, max_value_size=15
)


class TransferResult(Variant, total=False):
    Ok: nat64
    Err: "TransferError"


class TransferError(Variant, total=False):
    InsufficientBalance: nat64


@update
def transfer(to: Principal, amount: nat64) -> TransferResult:
    from_ = ic.caller()

    from_balance = accounts.get(from_) or 0

    if from_balance < amount:
        return {"Err": {"InsufficientBalance": from_balance}}

    to_balance = accounts.get(to) or 0

    accounts.insert(from_, from_balance - amount)
    accounts.insert(to, to_balance + amount)

    return {"Ok": amount}
```

`payout_canister`:

```python
from kybra import (
    Async,
    CallResult,
    match,
    nat64,
    Principal,
    Service,
    service_update,
    update,
    Variant,
)


class TokenCanister(Service):
    @service_update
    def transfer(self, to: Principal, amount: nat64) -> "TransferResult":
        ...


class TransferResult(Variant, total=False):
    Ok: nat64
    Err: "TransferError"


class TransferError(Variant, total=False):
    InsufficientBalance: nat64


token_canister = TokenCanister(Principal.from_str("r7inp-6aaaa-aaaaa-aaabq-cai"))


class PayoutResult(Variant, total=False):
    Ok: nat64
    Err: str


@update
def payout(to: Principal, amount: nat64) -> Async[PayoutResult]:
    call_result: CallResult[TransferResult] = yield token_canister.transfer(to, amount)

    def handle_transfer_result_ok(transfer_result: TransferResult) -> PayoutResult:
        return match(
            transfer_result,
            {
                "Ok": lambda ok: {"Ok": ok},
                "Err": lambda err: {"Err": str(err)},
            },
        )

    return match(
        call_result,
        {
            "Ok": handle_transfer_result_ok,
            "Err": lambda err: {"Err": err},
        },
    )
```

So far we have only shown a cross-canister call from an update method. Update methods can call other update methods or query methods (but not composite query methods as discussed below). If an update method calls a query method, that query method will be called in replicated mode. Replicated mode engages the consensus process, but for queries the state will still be discarded.

Cross-canister calls can also be initiated from query methods. These are known as composite queries, and in Kybra they are simply query methods that return a generator using the `Async` type. Composite queries can call other composite query methods and regular query methods. Composite queries cannot call update methods.

Here's an example of a composite query method:

```python
from kybra import (
    Async,
    CallResult,
    match,
    Principal,
    query,
    Service,
    service_query,
    Variant,
)


class SomeCanister(Service):
    @service_query
    def query_for_boolean(self) -> bool:
        ...


some_canister = SomeCanister(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))


class QuerySomeCanisterResult(Variant, total=False):
    Ok: bool
    Err: str


@query
def query_some_canister() -> Async[QuerySomeCanisterResult]:
    call_result: CallResult[bool] = yield some_canister.query_for_boolean()

    return match(
        call_result, {"Ok": lambda ok: {"Ok": ok}, "Err": lambda err: {"Err": err}}
    )
```

You can expect cross-canister calls within the same subnet to take up to a few seconds to complete, and cross-canister calls across subnets [take about double that time](https://forum.dfinity.org/t/can-i-run-multiple-inter-canister-update-calls-in-parallel/13115/6).

If you don't need to wait for your cross-canister call to return, you can use `notify`:

```python
from kybra import (
    null,
    Principal,
    query,
    RejectionCode,
    Service,
    service_update,
    Variant,
    void,
)


class SomeCanister(Service):
    @service_update
    def receive_notification(self) -> void:
        ...


some_canister = SomeCanister(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))


class ReceiveNotificationResult(Variant, total=False):
    Ok: null
    Err: RejectionCode


@query
def send_notification() -> ReceiveNotificationResult:
    return some_canister.receive_notification().notify()
```

If you need to send cycles with your cross-canister call, you can call `with_cycles` before calling `call` or `notify`:

```python
from kybra import (
    null,
    Principal,
    query,
    RejectionCode,
    Service,
    service_update,
    Variant,
    void,
)


class SomeCanister(Service):
    @service_update
    def receive_notification(self) -> void:
        ...


some_canister = SomeCanister(Principal.from_str("ryjl3-tyaaa-aaaaa-aaaba-cai"))


class ReceiveNotificationResult(Variant, total=False):
    Ok: null
    Err: RejectionCode


@query
def send_notification() -> ReceiveNotificationResult:
    return some_canister.receive_notification().with_cycles(1_000_000).notify()
```
