# pre upgrade

This section is a work in progress.

Examples:

-   [pre_and_post_upgrade](https://github.com/demergent-labs/kybra/tree/main/examples/pre_and_post_upgrade)

```python
from kybra import ic, pre_upgrade, void


@pre_upgrade
def pre_upgrade_() -> void:
    ic.print("This runs before every canister upgrade")
```
