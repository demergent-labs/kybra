import _random

from kybra import float64, update


@update
def random_number() -> float64:
    return _random.Random().random()
