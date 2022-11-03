from kybra import Async, blob, call, Canister, CanisterResult, Principal, update

# TODO rerun dfx generate once cross canister calls are fully implemented

class ManagementCanister(Canister):
    @call
    def raw_rand(self) -> blob: ...

@update
def get_randomness_directly() -> Async[blob]:
    management_canister = ManagementCanister(Principal.from_str('aaaaa-aa'))

    randomness_result: CanisterResult[blob] = yield management_canister.raw_rand()

    if randomness_result.ok is None:
        return bytes()

    return randomness_result.ok

@update
def get_randomness_indirectly() -> Async[blob]:
    indirect_randomness: blob = yield get_randomness()

    return indirect_randomness

@update
def get_randomness_super_indirectly() -> Async[blob]:
    randomness0: blob = yield get_randomness_level0()
    randomness1: blob = yield get_randomness_level1()
    randomness2: blob = yield get_randomness_level2()

    return randomness0 + randomness1 + randomness2

def get_randomness_level0() -> Async[blob]:
    randomness: blob = yield get_randomness_level1()
    return randomness

def get_randomness_level1() -> Async[blob]:
    randomness: blob = yield get_randomness_level2()
    return randomness

def get_randomness_level2() -> Async[blob]:
    randomness: blob = yield get_randomness()
    return randomness

def get_randomness() -> Async[blob]:
    management_canister = ManagementCanister(Principal.from_str('aaaaa-aa'))

    randomness_result: CanisterResult[blob] = yield management_canister.raw_rand()

    if randomness_result.ok is None:
        return bytes()

    return randomness_result.ok