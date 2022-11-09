def red(text: str) -> str:
    return f"\x1b[31m{text}\x1b[0m"


def yellow(text: str) -> str:
    return f"\x1b[33m{text}\x1b[0m"


def green(text: str) -> str:
    return f"\x1b[32m{text}\x1b[0m"


def blue(text: str) -> str:
    return f"\x1b[34m{text}\x1b[0m"


def purple(text: str) -> str:
    return f"\x1b[35m{text}\x1b[0m"


def dim(text: str) -> str:
    return f"\x1b[2m{text}\x1b[0m"
