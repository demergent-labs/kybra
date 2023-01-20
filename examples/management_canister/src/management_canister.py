from kybra import Async, blob, CanisterResult, nat, null, Principal, query, update
from kybra.canisters.management import CreateCanisterResult, CanisterStatusArgs, CanisterStatusResult, management_canister, ProvisionalCreateCanisterWithCyclesResult
from src.types import DefaultResult, ExecuteCreateCanisterResult, ExecuteProvisionalCreateCanisterWithCyclesResult, GetCanisterStatusResult, RawRandResult
from typing import TypedDict


class State(TypedDict):
    created_canister_id: Principal


state: State = {
    'created_canister_id': Principal.from_str('aaaaa-aa')
}


@update
def execute_create_canister() -> Async[ExecuteCreateCanisterResult]:
    create_canister_result_canister_result: CanisterResult[CreateCanisterResult] = yield management_canister.create_canister({
        'settings': None
    }).with_cycles(50_000_000_000_000)
    # TODO in Azle the amount we send is much smaller, I think something changed from dfx 11 to dfx 12, look into it

    if create_canister_result_canister_result.err is not None:
        return {
            'err': create_canister_result_canister_result.err
        }

    create_canister_result = create_canister_result_canister_result.ok

    state['created_canister_id'] = create_canister_result['canister_id']

    return {
        'ok': create_canister_result
    }


@update
def execute_update_settings(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CanisterResult[null] = yield management_canister.update_settings({
        'canister_id': canister_id,
        'settings': {
            'controllers': None,
            'compute_allocation': 1,
            'memory_allocation': 3_000_000,
            'freezing_threshold': 2_000_000
        }
    })

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': True
    }


@update
def execute_install_code(canister_id: Principal, wasm_module: blob) -> Async[DefaultResult]:
    canister_result: CanisterResult[null] = yield management_canister.install_code({
        'mode': {
            'install': None
        },
        'canister_id': canister_id,
        'wasm_module': wasm_module,
        'arg': bytes()
    }).with_cycles(100_000_000_000)

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': True
    }


@update
def execute_uninstall_code(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CanisterResult[null] = yield management_canister.uninstall_code({
        'canister_id': canister_id
    })

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': True
    }


@update
def execute_start_canister(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CanisterResult[null] = yield management_canister.start_canister({
        'canister_id': canister_id
    })

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': True
    }


@update
def execute_stop_canister(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CanisterResult[null] = yield management_canister.stop_canister({
        'canister_id': canister_id
    })

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': True
    }


@update
def get_canister_status(args: CanisterStatusArgs) -> Async[GetCanisterStatusResult]:
    canister_status_result_canister_result: CanisterResult[CanisterStatusResult] = yield management_canister.canister_status({
        'canister_id': args['canister_id']
    })

    if canister_status_result_canister_result.err is not None:
        return {
            'err': canister_status_result_canister_result.err
        }

    canister_status_result = canister_status_result_canister_result.ok

    return {
        'ok': canister_status_result
    }


@update
def execute_delete_canister(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CanisterResult[null] = yield management_canister.delete_canister({
        'canister_id': canister_id
    })

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': True
    }

# TODO execute_deposit_cycles should be implemented in Azle and Kybra


@update
def get_raw_rand() -> Async[RawRandResult]:
    raw_rand_canister_result: CanisterResult[blob] = yield management_canister.raw_rand()

    if raw_rand_canister_result.err is not None:
        return {
            'err': raw_rand_canister_result.err
        }

    randomness = raw_rand_canister_result.ok

    return {
        'ok': randomness
    }

# TODO needs tests


@update
def provisional_create_canister_with_cycles() -> Async[ExecuteProvisionalCreateCanisterWithCyclesResult]:
    canister_result: CanisterResult[ProvisionalCreateCanisterWithCyclesResult] = yield management_canister.provisional_create_canister_with_cycles({
        'amount': None,
        'settings': None
    })

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    provisional_create_canister_with_cycles_result = canister_result.ok

    return {
        'ok': provisional_create_canister_with_cycles_result
    }


@update
def provisional_top_up_canister(canister_id: Principal, amount: nat) -> Async[DefaultResult]:
    canister_result: CanisterResult[null] = yield management_canister.provisional_top_up_canister({
        'canister_id': canister_id,
        'amount': amount
    })

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    return {
        'ok': True
    }


@query
def get_created_canister_id() -> Principal:
    return state['created_canister_id']
