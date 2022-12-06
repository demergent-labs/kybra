from kybra import Async, Canister, CanisterResult, ic, init, method, post_upgrade, Principal, query, update, Variant


class WhoAmICanister(Canister):
    @method
    def installer(self) -> Principal: ...

    @method
    def argument(self) -> Principal: ...

    @method
    def whoami(self) -> Principal: ...

    @method
    def id(self) -> Principal: ...

    @method
    def idQuick(self) -> Principal: ...


class WhoAmIResult(Variant, total=False):
    ok: Principal
    err: str


install: Principal = Principal.from_str('aaaaa-aa')
someone: Principal = Principal.from_str('aaaaa-aa')


@init
def init_(somebody: Principal):
    global install
    global someone

    install = ic.caller()
    someone = somebody


@post_upgrade
def post_upgrade_(somebody: Principal):
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
