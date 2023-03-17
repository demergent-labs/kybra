from kybra import Async, Canister, CanisterResult, ic, init, post_upgrade, Principal, query, service_query, service_update, update, Variant, void


class WhoAmICanister(Canister):
    @service_query
    def installer(self) -> Principal: ...

    @service_query
    def argument(self) -> Principal: ...

    @service_update
    def whoami(self) -> Principal: ...

    @service_update
    def id(self) -> Principal: ...

    @service_query
    def idQuick(self) -> Principal: ...


class WhoAmIResult(Variant, total=False):
    ok: Principal
    err: str


install: Principal = Principal.from_str('aaaaa-aa')
someone: Principal = Principal.from_str('aaaaa-aa')


@init
def init_(somebody: Principal) -> void:
    global install
    global someone

    install = ic.caller()
    someone = somebody


@post_upgrade
def post_upgrade_(somebody: Principal) -> void:
    global install
    global someone

    install = ic.caller()
    someone = somebody


@query
def installer() -> Principal:
    return install


@query
def argument() -> Principal:
    return someone


@update
def whoami() -> Principal:
    return ic.caller()


@update
def id() -> Async[Principal]:
    thisCanister = WhoAmICanister(ic.id())
    result: CanisterResult[Principal] = yield thisCanister.whoami()
    return result.ok if result.ok is not None else Principal.from_str('aaaaa-aa')


@query
def id_quick() -> Principal:
    return ic.id()
