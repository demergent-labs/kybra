from kybra import blob, empty, ic, int8, manual, nat, nat64, nat32, opt, Principal, query, update

# returns the argument data as bytes.
@query
def arg_data_raw(
    arg1: blob,
    arg2: int8,
    arg3: bool,
    arg4: str
) -> blob:
    return ic.arg_data_raw()

# returns the length of the argument data in bytes
@query
def arg_data_raw_size(
    arg1: blob,
    arg2: int8,
    arg3: bool,
    arg4: str
) -> nat32:
    return ic.arg_data_raw_size()

# returns the principal of the identity that called this function
@query
def caller() -> Principal:
    return ic.caller()

# returns the amount of cycles available in the canister
@query
def canister_balance() -> nat64:
    return ic.canister_balance()

# returns the amount of cycles available in the canister
@query
def canister_balance128() -> nat:
    return ic.canister_balance128()

# When called from a query call, returns the data certificate authenticating certified_data set by this canister. Returns None if called not from a query call.
@query
def data_certificate() -> opt[blob]:
    return ic.data_certificate()

# When called from a query call, returns the data certificate authenticating certified_data set by this canister. Returns None if called not from a query call.
@update
def data_certificate_null() -> opt[blob]:
    return ic.data_certificate()

# returns this canister's id
@query
def id() -> Principal:
    return ic.id()

@query
def performance_counter() -> nat64:
    return ic.performance_counter(0)

# prints a message through the local replica's output
@query
def print(message: str) -> bool:
    ic.print(message)

    return True

@query
def reject(message: str) -> manual[empty]:
    ic.reject(message)

# sets up to 32 bytes of certified data
@update
def set_certified_data(data: blob):
    ic.set_certified_data(data)

# returns the current timestamp
@query
def time() -> nat64:
    return ic.time()

# traps with a message, stopping execution and discarding all state within the call
@query
def trap(message: str) -> bool:
    ic.trap(message)

    return True
