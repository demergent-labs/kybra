# type: ignore

from wasmer import (
    Store,
    Module,
    Instance,
    Function,
    FunctionType,
    Type,
    wasi,
)


def generate_candid_file(paths, canister_name) -> str:
    file = open(f"{paths['canister']}/{canister_name}_app.wasm", "rb")
    wasm_buffer = file.read()
    file.close()

    store = Store()

    module = Module(store, wasm_buffer)

    wasi_env = wasi.StateBuilder(paths["wasm"]).finalize()
    import_object = wasi_env.generate_import_object(store, wasi.Version.LATEST)

    import_object.register(
        "ic0",
        {
            "msg_reply": Function(
                store, lambda *args, **kwargs: None, FunctionType([], [])
            ),
            "stable_size": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "stable64_size": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I64]),
            ),
            "stable_write": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32, Type.I32], []),
            ),
            "stable_read": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32, Type.I32], []),
            ),
            "debug_print": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32], []),
            ),
            "trap": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32], []),
            ),
            "time": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I64]),
            ),
            "msg_caller_size": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "msg_caller_copy": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32, Type.I32], []),
            ),
            "canister_self_size": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "canister_self_copy": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32, Type.I32], []),
            ),
            "canister_cycle_balance": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I64]),
            ),
            "canister_cycle_balance128": Function(
                store, lambda *args, **kwargs: None, FunctionType([Type.I32], [])
            ),
            "canister_version": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I64]),
            ),
            "certified_data_set": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32], []),
            ),
            "data_certificate_present": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "data_certificate_size": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "data_certificate_copy": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32, Type.I32], []),
            ),
            "msg_reply_data_append": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32], []),
            ),
            "call_cycles_add128": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I64, Type.I64], []),
            ),
            "call_new": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType(
                    [
                        Type.I32,
                        Type.I32,
                        Type.I32,
                        Type.I32,
                        Type.I32,
                        Type.I32,
                        Type.I32,
                        Type.I32,
                    ],
                    [],
                ),
            ),
            "call_data_append": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32], []),
            ),
            "call_perform": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "call_cycles_add": Function(
                store, lambda *args, **kwargs: None, FunctionType([Type.I64], [])
            ),
            "call_on_cleanup": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32], []),
            ),
            "msg_reject_code": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "msg_reject_msg_size": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "msg_reject_msg_copy": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32, Type.I32], []),
            ),
            "msg_reject": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32], []),
            ),
            "msg_cycles_available": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I64]),
            ),
            "msg_cycles_available128": Function(
                store, lambda *args, **kwargs: None, FunctionType([Type.I32], [])
            ),
            "msg_cycles_refunded": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I64]),
            ),
            "msg_cycles_refunded128": Function(
                store, lambda *args, **kwargs: None, FunctionType([Type.I32], [])
            ),
            "msg_cycles_accept": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([Type.I64], [Type.I64]),
            ),
            "msg_cycles_accept128": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I64, Type.I64, Type.I32], []),
            ),
            "msg_arg_data_size": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "msg_arg_data_copy": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32, Type.I32], []),
            ),
            "accept_message": Function(
                store, lambda *args, **kwargs: None, FunctionType([], [])
            ),
            "msg_method_name_size": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([], [Type.I32]),
            ),
            "msg_method_name_copy": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I32, Type.I32, Type.I32], []),
            ),
            "performance_counter": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([Type.I32], [Type.I64]),
            ),
            "stable_grow": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([Type.I32], [Type.I32]),
            ),
            "stable64_grow": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([Type.I64], [Type.I64]),
            ),
            "stable64_write": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I64, Type.I64, Type.I64], []),
            ),
            "stable64_read": Function(
                store,
                lambda *args, **kwargs: None,
                FunctionType([Type.I64, Type.I64, Type.I64], []),
            ),
            "global_timer_set": Function(
                store,
                lambda *args, **kwargs: 0,
                FunctionType([Type.I64], [Type.I64]),
            ),
        },
    )

    instance = Instance(module, import_object)

    candid_pointer = instance.exports.get_candid_pointer()

    memory = instance.exports.memory.uint8_view()
    string_bytes = []
    i = candid_pointer
    while memory[i] != 0:
        string_bytes.append(memory[i])
        i += 1
    candid_string = bytes(string_bytes).decode("utf-8")

    return candid_string
