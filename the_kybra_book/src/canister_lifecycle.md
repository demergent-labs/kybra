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
