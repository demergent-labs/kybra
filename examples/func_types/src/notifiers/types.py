from kybra import blob, Func, Oneway, Service, service_query, void

NotifierFunc = Func(Oneway[[blob], void])


class Notifier(Service):
    @service_query
    def get_notifier(self) -> NotifierFunc:
        ...
