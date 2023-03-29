from kybra import Async, blob, Canister, CanisterResult, Principal, heartbeat, query, service_update, void

initialized: blob = bytes()


class ManagementCanister(Canister):
    @service_update
    def raw_rand(self) -> blob: ...


@heartbeat
def heartbeat_() -> Async[void]:
    global initialized
    management_canister = ManagementCanister(Principal.from_str('aaaaa-aa'))
    randomness_result: CanisterResult[blob] = yield management_canister.raw_rand()

    if randomness_result.Err is not None:
        initialized = bytes()

    initialized = randomness_result.Ok


@query
def get_initialized() -> blob:
    return initialized
