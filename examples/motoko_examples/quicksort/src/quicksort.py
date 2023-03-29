from typing import TypeVar, Callable
from comparison import isLess, isGreater

X = TypeVar("X")


def sort_by(xs: list[X], f: Callable[[X, X], int]) -> list[X]:
    n = len(xs)
    if n < 2:
        return xs
    else:
        result = xs.copy()
        sort_by_helper(result, 0, n - 1, f)
        return result


def sort_by_helper(xs: list[X], l: int, r: int, f: Callable[[X, X], int]):
    if l < r:
        i = l
        j = r
        swap = xs[0]
        pivot = xs[round(abs(l + r) / 2)]
        while (i <= j):
            while (isLess(f(xs[abs(i)], pivot))):
                i += 1

            while (isGreater(f(xs[abs(j)], pivot))):
                j -= 1

            if (i <= j):
                swap = xs[abs(i)]
                xs[abs(i)] = xs[abs(j)]
                xs[abs(j)] = swap
                i += 1
                j -= 1

        if (l < j):
            sort_by_helper(xs, l, j, f)

        if (i < r):
            sort_by_helper(xs, i, r, f)
