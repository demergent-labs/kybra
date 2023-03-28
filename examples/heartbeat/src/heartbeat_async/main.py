from kybra import (
    Async,
    blob,
    CanisterResult,
    match,
    Principal,
    heartbeat,
    query,
    Service,
    service_update,
    void,
)

initialized: blob = bytes()


class ManagementCanister(Service):
    @service_update
    def raw_rand(self) -> blob:
        ...


@heartbeat
def heartbeat_() -> Async[void]:
    management_canister = ManagementCanister(Principal.from_str("aaaaa-aa"))

    randomness_result: CanisterResult[blob] = yield management_canister.raw_rand()

    def handle_ok(ok: blob) -> void:
        global initialized
        initialized = ok

    def handle_err(_) -> void:
        global initialized
        initialized = bytes()

    match(randomness_result, {"Ok": handle_ok, "Err": handle_err})


@query
def get_initialized() -> blob:
    return initialized
