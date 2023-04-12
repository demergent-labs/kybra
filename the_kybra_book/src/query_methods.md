# Query Methods

## TLDR

-   Decorate functions with `@query`
-   Read-only
-   Executed on a single node
-   No consensus
-   Latency on the order of ~100 milliseconds
-   5 billion Wasm instruction limit
-   4 GiB heap limit
-   ~32k queries per second per canister

The most basic way to expose your canister's functionality publicly is through a query method. Here's an example of a simple query method:

```python
from kybra import query


@query
def get_string() -> str:
    return "This is a query method!"
```

`get_string` can be called from the outside world through the IC's HTTP API. You'll usually invoke this API from the [`dfx command line`, `dfx web UI`, or an agent](./deployment.md#interacting-with-your-canister).

From the `dfx command line` you can call it like this:

```bash
dfx canister call my_canister get_string
```

Query methods are read-only. They do not persist any state changes. Take a look at the following example:

```python
from kybra import query, void

db = {}


@query
def set(key: str, value: str) -> void:
    db[key] = value
```

Calling `set` will perform the operation of setting the `key` property on the `db` object to `value`, but after the call finishes that change will be discarded.

This is because query methods are executed on a single node machine and do not go through [consensus](https://internetcomputer.org/how-it-works/consensus/). This results in lower latencies, perhaps on the order of 100 milliseconds.

There is a limit to how much computation can be done in a single call to a query method. The current query call limit is [5 billion Wasm instructions](https://internetcomputer.org/docs/current/developer-docs/production/instruction-limits). Here's an example of a query method that runs the risk of reaching the limit:

```python
from kybra import nat32, query


@query
def pyramid(levels: nat32) -> str:
    levels_array = [0 for _ in range(levels)]
    asterisk_array = [
        ["*" for _ in range(i + 1)] + ["\n"] for i in range(len(levels_array))
    ]
    flattened_array = [element for subarray in asterisk_array for element in subarray]

    return "".join(flattened_array)

```

From the `dfx command line` you can call `pyramid` like this:

```bash
dfx canister call my_canister pyramid '(600)'
```

With an argument of `600`, `pyramid` will fail with an error `...exceeded the instruction limit for single message execution`.

Keep in mind that each query method invocation has up to 4 GiB of heap available.

In terms of query scalability, an individual canister [likely has an upper bound of ~36k queries per second](https://forum.dfinity.org/t/what-is-the-theroretical-number-for-txns-per-second-on-internet-computer-right-now/14039/6).
