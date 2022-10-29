from kybra import query, update

notified: bool = False

# TODO remove the return type once no-return/void is possible, to match the Azle example
@update
def receive_notification() -> bool:
    global notified
    notified = True

    return True

@query
def get_notified() -> bool:
    return notified
