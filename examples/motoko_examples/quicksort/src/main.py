from kybra import update
from comparison import compare
import quicksort


@update
def sort(xs: list[int]) -> list[int]:
    return quicksort.sort_by(xs, compare)
