# post upgrade

This section is a work in progress.

Examples:

-   [pre_and_post_upgrade](https://github.com/demergent-labs/kybra/tree/main/examples/pre_and_post_upgrade)
-   [whoami](https://github.com/demergent-labs/kybra/tree/main/examples/motoko_examples/whoami)

```python
from kybra import ic, post_upgrade, void


@post_upgrade
def post_upgrade_() -> void:
    ic.print("This runs after every canister upgrade")
```
