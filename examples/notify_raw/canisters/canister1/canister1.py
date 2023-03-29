from kybra import ic, Principal, RejectionCode, update, Variant


class SendNotificationResult(Variant, total=False):
    Ok: bool
    Err: RejectionCode


@update
def send_notification() -> SendNotificationResult:
    result = ic.notify_raw(
        Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'),
        'receive_notification',
        ic.candid_encode('()'),
        0
    )

    if 'Err' in result:
        return {
            'Err': result['Err']
        }

    return {
        'Ok': True
    }
