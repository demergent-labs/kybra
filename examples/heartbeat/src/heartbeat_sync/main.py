from kybra import heartbeat, query, void

initialized: bool = False


@heartbeat
def heartbeat_() -> void:
    global initialized
    initialized = True


@query
def get_initialized() -> bool:
    return initialized
