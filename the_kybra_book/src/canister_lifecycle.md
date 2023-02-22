# Canister Lifecycle

This chapter is a work in progress.

TODO check that the underscores are necessary

TODO check on explicit void

```python
from kybra import init, post_upgrade, pre_upgrade

@init
def init_():
    print('runs on first canister install')

@pre_upgrade
def pre_upgrade_():
    print('runs before canister upgrade')

@post_upgrade_
def post_upgrade_():
    print('runs after canister upgrade')
```
