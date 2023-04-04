from kybra import (
    Async,
    blob,
    CallResult,
    match,
    nat,
    Principal,
    query,
    update,
    void,
)
from kybra.canisters.management import (
    CreateCanisterResult,
    CanisterStatusArgs,
    CanisterStatusResult,
    management_canister,
    ProvisionalCreateCanisterWithCyclesResult,
)
from src.types import (
    DefaultResult,
    ExecuteCreateCanisterResult,
    ExecuteProvisionalCreateCanisterWithCyclesResult,
    GetCanisterStatusResult,
    RawRandResult,
)
from typing import TypedDict


class State(TypedDict):
    created_canister_id: Principal


state: State = {"created_canister_id": Principal.from_str("aaaaa-aa")}


@update
def execute_create_canister() -> Async[ExecuteCreateCanisterResult]:
    create_canister_result_canister_result: CallResult[
        CreateCanisterResult
    ] = yield management_canister.create_canister({"settings": None}).with_cycles(
        50_000_000_000_000
    )
    # TODO in Azle the amount we send is much smaller, I think something changed from dfx 11 to dfx 12, look into it

    def handle_ok(
        create_canister_result: CreateCanisterResult,
    ) -> ExecuteCreateCanisterResult:
        state["created_canister_id"] = create_canister_result["canister_id"]

        return {"Ok": create_canister_result}

    return match(
        create_canister_result_canister_result,
        {"Ok": handle_ok, "Err": lambda err: {"Err": err}},
    )


@update
def execute_update_settings(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CallResult[void] = yield management_canister.update_settings(
        {
            "canister_id": canister_id,
            "settings": {
                "controllers": None,
                "compute_allocation": 1,
                "memory_allocation": 3_000_000,
                "freezing_threshold": 2_000_000,
            },
        }
    )

    return match(
        canister_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )


@update
def execute_install_code(
    canister_id: Principal, wasm_module: blob
) -> Async[DefaultResult]:
    canister_result: CallResult[void] = yield management_canister.install_code(
        {
            "mode": {"install": None},
            "canister_id": canister_id,
            "wasm_module": wasm_module,
            "arg": bytes(),
        }
    ).with_cycles(100_000_000_000)

    return match(
        canister_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )


@update
def execute_uninstall_code(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CallResult[void] = yield management_canister.uninstall_code(
        {"canister_id": canister_id}
    )

    return match(
        canister_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )


@update
def execute_start_canister(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CallResult[void] = yield management_canister.start_canister(
        {"canister_id": canister_id}
    )

    return match(
        canister_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )


@update
def execute_stop_canister(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CallResult[void] = yield management_canister.stop_canister(
        {"canister_id": canister_id}
    )

    return match(
        canister_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )


@update
def get_canister_status(args: CanisterStatusArgs) -> Async[GetCanisterStatusResult]:
    canister_status_result_canister_result: CallResult[
        CanisterStatusResult
    ] = yield management_canister.canister_status({"canister_id": args["canister_id"]})

    return match(
        canister_status_result_canister_result,
        {
            "Ok": lambda canister_status_result: {"Ok": canister_status_result},
            "Err": lambda err: {"Err": err},
        },
    )


@update
def execute_delete_canister(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CallResult[void] = yield management_canister.delete_canister(
        {"canister_id": canister_id}
    )

    return match(
        canister_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )


@update
def execute_deposit_cycles(canister_id: Principal) -> Async[DefaultResult]:
    canister_result: CallResult[void] = yield management_canister.deposit_cycles(
        {"canister_id": canister_id}
    ).with_cycles(1_000_000)

    return match(
        canister_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )


@update
def get_raw_rand() -> Async[RawRandResult]:
    raw_rand_canister_result: CallResult[blob] = yield management_canister.raw_rand()

    return match(
        raw_rand_canister_result,
        {
            "Ok": lambda randomness: {"Ok": randomness},
            "Err": lambda err: {"Err": err},
        },
    )


# TODO needs tests


@update
def provisional_create_canister_with_cycles() -> (
    Async[ExecuteProvisionalCreateCanisterWithCyclesResult]
):
    canister_result: CallResult[
        ProvisionalCreateCanisterWithCyclesResult
    ] = yield management_canister.provisional_create_canister_with_cycles(
        {"amount": None, "settings": None}
    )

    return match(
        canister_result,
        {
            "Ok": lambda provisional_create_canister_with_cycles_result: {
                "Ok": provisional_create_canister_with_cycles_result
            },
            "Err": lambda err: {"Err": err},
        },
    )


@update
def provisional_top_up_canister(
    canister_id: Principal, amount: nat
) -> Async[DefaultResult]:
    canister_result: CallResult[
        void
    ] = yield management_canister.provisional_top_up_canister(
        {"canister_id": canister_id, "amount": amount}
    )

    return match(
        canister_result, {"Ok": lambda _: {"Ok": True}, "Err": lambda err: {"Err": err}}
    )


@query
def get_created_canister_id() -> Principal:
    return state["created_canister_id"]
