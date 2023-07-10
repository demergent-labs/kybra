from kybra import ic, init, Principal, RejectionCode, update, Variant, void


class SendNotificationResult(Variant, total=False):
    Ok: bool
    Err: RejectionCode


canister2_principal = Principal.from_str("aaaaa-aa")


@init
def init_(canister2_id: Principal) -> void:
    global canister2_principal

    canister2_principal = canister2_id


@update
def send_notification() -> SendNotificationResult:
    result = ic.notify_raw(
        canister2_principal,
        "receive_notification",
        ic.candid_encode("()"),
        0,
    )

    if "Err" in result:
        return {"Err": result["Err"]}

    return {"Ok": True}
