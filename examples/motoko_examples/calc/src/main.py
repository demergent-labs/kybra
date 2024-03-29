from kybra import Opt, update, void

cell: int = 0


@update
def add(n: int) -> int:
    global cell

    cell += n

    return cell


@update
def sub(n: int) -> int:
    global cell

    cell -= n

    return cell


@update
def mul(n: int) -> int:
    global cell

    cell *= n

    return cell


@update
def div(n: int) -> Opt[int]:
    global cell

    if n == 0:
        result = None
    else:
        cell //= n
        result = cell

    return result


@update
def clearall() -> void:
    global cell

    cell = 0
