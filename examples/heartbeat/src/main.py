from kybra import heartbeat, query;

initialized: bool = False

@heartbeat
def heartbeat_():
    global initialized
    initialized = True

@query
def get_initialized() -> bool:
    return initialized
