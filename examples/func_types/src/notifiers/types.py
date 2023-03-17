from kybra import blob, Func, Canister, Oneway, service_query, void

NotifierFunc = Func(Oneway[[blob], void])


class Notifier(Canister):
    @service_query
    def get_notifier(self) -> NotifierFunc:
        ...
