from kybra import ic, Principal, update, Variant

class SendNotificationResult(Variant, total=False):
    ok: bool
    err: str

@update
def send_notification() -> SendNotificationResult:
    result = ic.notify_raw(
        Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'),
        'receive_notification',
        ic.candid_encode('()'),
        0
    )

    if 'err' in result:
        return {
            'err': result['err']
        }

    return {
        'ok': True
    }
