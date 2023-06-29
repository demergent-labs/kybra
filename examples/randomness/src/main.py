import random

from kybra import float64, update


@update
def random_number() -> float64:
    return random.random()
