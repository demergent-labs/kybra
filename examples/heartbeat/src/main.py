from kybra import heartbeat, query;

initialized: bool = False

@heartbeat
def heartbeat_():
    global initialized
    initialized = True

@query
def getInitialized() -> bool:
    return initialized
