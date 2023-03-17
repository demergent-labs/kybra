# Canister Lifecycle

This chapter is a work in progress.

```python
from kybra import init, post_upgrade, pre_upgrade

@init
def init_() -> void:
    print('runs on first canister install')

@pre_upgrade
def pre_upgrade_() -> void:
    print('runs before canister upgrade')

@post_upgrade
def post_upgrade_() -> void:
    print('runs after canister upgrade')
```
