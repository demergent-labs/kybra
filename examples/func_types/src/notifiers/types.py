from kybra import blob, Func, Canister, Oneway, query, void

NotifierFunc = Func(Oneway[[blob], void])


class Notifier(Canister):
    @query
    def get_notifier(self) -> NotifierFunc:
        ...
