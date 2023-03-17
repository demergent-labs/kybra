from kybra import GuardResult, ic, manual, update, query


def unpassable() -> GuardResult:
    ic.print("We are in the unpassable")
    return {
        'err': "You shall not pass!"
    }


def adelante() -> GuardResult:
    ic.print("We are in the adelante")
    return {
        'ok': None,
    }


@query(guard=adelante)
def accessible() -> bool:
    return True


@query(guard=adelante)
def guarded_manual() -> manual[bool]:
    ic.reply(True)


@query(guard=unpassable)
def inaccessible() -> bool:
    return False


@update(guard=unpassable)
def inaccessible_update() -> bool:
    return False


@query
def unguarded() -> bool:
    return True
