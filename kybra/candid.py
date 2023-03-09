# type: ignore

import wasmtime


def generate_candid_file(paths) -> str:

    file = open(paths["wasm"], "rb")
    wasm_buffer = file.read()
    file.close()

    store = wasmtime.Store()

    module = wasmtime.Module(store.engine, wasm_buffer)

    msg_reply = wasmtime.Func(store, wasmtime.FuncType([], []), None)
    stable_size = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    stable64_size = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i64()]), None
    )
    stable_write = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    stable_read = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    debug_print = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32()], []),
        None,
    )
    trap = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32()], []),
        None,
    )
    time = wasmtime.Func(store, wasmtime.FuncType([], [wasmtime.ValType.i64()]), None)
    msg_caller_size = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    msg_caller_copy = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    canister_self_size = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    canister_self_copy = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    canister_cycle_balance = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i64()]), None
    )
    canister_cycle_balance128 = wasmtime.Func(
        store, wasmtime.FuncType([wasmtime.ValType.i32()], []), None
    )
    certified_data_set = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32()], []),
        None,
    )
    data_certificate_present = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    data_certificate_size = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    data_certificate_copy = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    msg_reply_data_append = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32()], []),
        None,
    )
    call_cycles_add128 = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i64(), wasmtime.ValType.i64()], []),
        None,
    )
    call_new = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [
                wasmtime.ValType.i32(),
                wasmtime.ValType.i32(),
                wasmtime.ValType.i32(),
                wasmtime.ValType.i32(),
                wasmtime.ValType.i32(),
                wasmtime.ValType.i32(),
                wasmtime.ValType.i32(),
                wasmtime.ValType.i32(),
            ],
            [],
        ),
        None,
    )
    call_data_append = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32()], []),
        None,
    )
    call_perform = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    call_cycles_add = wasmtime.Func(
        store, wasmtime.FuncType([wasmtime.ValType.i64()], []), None
    )
    call_on_cleanup = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32()], []),
        None,
    )
    msg_reject_code = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    msg_reject_msg_size = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    msg_reject_msg_copy = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    msg_reject = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32(), wasmtime.ValType.i32()], []),
        None,
    )
    msg_cycles_available = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i64()]), None
    )
    msg_cycles_available128 = wasmtime.Func(
        store, wasmtime.FuncType([wasmtime.ValType.i32()], []), None
    )
    msg_cycles_refunded = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i64()]), None
    )
    msg_cycles_refunded128 = wasmtime.Func(
        store, wasmtime.FuncType([wasmtime.ValType.i32()], []), None
    )
    msg_cycles_accept = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i64()], [wasmtime.ValType.i64()]),
        None,
    )
    msg_cycles_accept128 = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i64(), wasmtime.ValType.i64(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    msg_arg_data_size = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    msg_arg_data_copy = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    accept_message = wasmtime.Func(store, wasmtime.FuncType([], []), None)
    msg_method_name_size = wasmtime.Func(
        store, wasmtime.FuncType([], [wasmtime.ValType.i32()]), None
    )
    msg_method_name_copy = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i32(), wasmtime.ValType.i32(), wasmtime.ValType.i32()], []
        ),
        None,
    )
    performance_counter = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32()], [wasmtime.ValType.i64()]),
        None,
    )
    stable_grow = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i32()], [wasmtime.ValType.i32()]),
        None,
    )
    stable64_grow = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i64()], [wasmtime.ValType.i64()]),
        None,
    )
    stable64_write = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i64(), wasmtime.ValType.i64(), wasmtime.ValType.i64()], []
        ),
        None,
    )
    stable64_read = wasmtime.Func(
        store,
        wasmtime.FuncType(
            [wasmtime.ValType.i64(), wasmtime.ValType.i64(), wasmtime.ValType.i64()], []
        ),
        None,
    )
    global_timer_set = wasmtime.Func(
        store,
        wasmtime.FuncType([wasmtime.ValType.i64()], [wasmtime.ValType.i64()]),
        None,
    )

    instance = wasmtime.Instance(
        store,
        module,
        [
            msg_reply,
            stable_size,
            stable64_size,
            stable_write,
            stable_read,
            debug_print,
            trap,
            time,
            msg_caller_size,
            msg_caller_copy,
            canister_self_size,
            canister_self_copy,
            canister_cycle_balance,
            canister_cycle_balance128,
            certified_data_set,
            data_certificate_present,
            data_certificate_size,
            data_certificate_copy,
            msg_reply_data_append,
            call_cycles_add128,
            call_new,
            call_data_append,
            call_perform,
            call_cycles_add,
            call_on_cleanup,
            msg_reject_code,
            msg_reject_msg_size,
            msg_reject_msg_copy,
            msg_reject,
            msg_cycles_available,
            msg_cycles_available128,
            msg_cycles_refunded,
            msg_cycles_refunded128,
            msg_cycles_accept,
            msg_cycles_accept128,
            msg_arg_data_size,
            msg_arg_data_copy,
            accept_message,
            msg_method_name_size,
            msg_method_name_copy,
            performance_counter,
            stable_grow,
            stable64_grow,
            stable64_write,
            stable64_read,
            global_timer_set,
        ],
    )

    candid_pointer = instance.exports(store)["_cdk_get_candid_pointer"](store)
    candid_length = instance.exports(store)["_cdk_get_candid_length"](store)

    candid_bytes = bytes(
        instance.exports(store)["memory"].data_ptr(store)[
            candid_pointer : candid_pointer + candid_length
        ]
    )

    candid_string = candid_bytes.decode("utf-8")

    return candid_string
