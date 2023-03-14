# Gotchas and Caveats

This chapter is a work in progress.

-   Most of the `stdlib` doesn't work yet
-   Most PyPI packages will not work yet
-   All Kybra canisters have a public metadata key-value pair named `cdk` which contains the version of Kybra the canister is using
    -   Anyone can query this metadata which may be a security risk
-   [The datetime stdlib](https://docs.python.org/3/library/datetime.html) does not work yet, [see here for an alternative](https://github.com/demergent-labs/kybra/blob/main/examples/date/src/main.py)

## Python stdlib

There is limited support for the `stdlib`. The following modules may be supported as far as [RustPython](https://github.com/RustPython/RustPython) or the IC support them:

-   array
-   binascii
-   \_bisect
-   cmath
-   \_contextvars
-   \_csv
-   \_dis
-   gc
-   hashlib
-   \_json
-   math
-   pyexpat
-   \_struct
-   \_random
-   \_statistics
-   unicodedata
-   zlib

## Python External Packages

Installing external packages, such as from [PyPI](https://pypi.org/), will probably not work. The biggest problem you are most likely to run into is lack of support for most of the Python stdlib. Once the Wasm binary limit is increased on the IC, much more of the stdlib should be supported which should open up many external packages.

Once the majority of the stdlib is supported, you are likely to run into issues with the environment that the Python code is running in. It is a Rust `wasm32-unknown-unknown` environment that has access to the IC APIs. Any external packages that can't compile or run in this environment with the supported APIs will not work.

Much of the future work for enabling external packages will be forking and patching packages to support IC APIs.
