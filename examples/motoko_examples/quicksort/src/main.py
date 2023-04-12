from kybra import update, Vec
from comparison import compare
import quicksort


@update
def sort(xs: Vec[int]) -> Vec[int]:
    return quicksort.sort_by(xs, compare)
