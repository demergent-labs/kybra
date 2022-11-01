from kybra import Async, blob, CanisterResult, ic, Principal, update

# TODO we really need to use the ManagementCanister to get this to work appropriately, because of the candid decoding
# TODO swap out ic.call_raw for the ManagementCanister once we have general cross-canister calls enabled

@update
def get_randomness_directly() -> Async[blob]:
    randomness_result: CanisterResult[blob, str] = yield ic.call_raw(
        Principal.from_str('aaaaa-aa'),
        'raw_rand',
        ic.candid_encode('()'),
        0
    )

    if randomness_result[0] is None:
        return bytes()

    return randomness_result[0]

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
    randomness_result: CanisterResult[blob, str] = yield ic.call_raw(
        Principal.from_str('aaaaa-aa'),
        'raw_rand',
        ic.candid_encode('()'),
        0
    )

    if randomness_result[0] is None:
        return bytes()

    return randomness_result[0]
