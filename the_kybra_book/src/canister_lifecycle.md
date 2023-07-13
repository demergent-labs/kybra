# Canister Lifecycle

This chapter is a work in progress.

```python
from kybra import ic, init, post_upgrade, pre_upgrade, void


@init
def init_() -> void:
    ic.print("runs on first canister install")


@pre_upgrade
def pre_upgrade_() -> void:
    ic.print("runs before canister upgrade")


@post_upgrade
def post_upgrade_() -> void:
    ic.print("runs after canister upgrade")
```

## Caveats

### params

Keep in mind [this caveat](./caveats.md#init-and-post_upgrade-params) when working with `init` or `post_upgrade` params.

### ic.caller()

Usually `ic.caller()` if called from `init` or `post_upgrade` is the principal of your local dfx identity. In Kybra canisters `ic.caller()` if called from `init` or `post_upgrade` is the canister's own principal.

### guard functions

`init` and `post_upgrade` cannot have guard functions applied to them.
