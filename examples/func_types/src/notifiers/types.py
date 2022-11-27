from typing import TypeAlias
from kybra import blob, Func, Canister, method, Oneway

NotifierFunc: TypeAlias = Func(Oneway[[blob], None])


class Notifier(Canister):
    @method
    def get_notifier(self) -> NotifierFunc:
        ...
