import time
from typing import Any, Callable, TypeVar

from kybra.colors import dim

T = TypeVar('T')

def timed(func: Callable[..., T]) -> Callable[..., T]:
    def inside(*args: Any) -> T:
        start_time = time.time()
        return_value = func(*args)
        end_time = time.time()
        duration = end_time - start_time

        print(f"\nDone in {round(duration, 2)}s.")

        return return_value
    return inside

def timed_inline(func: Callable[..., T]) -> Callable[..., T]:
    def inside(*args: Any, verbose: bool=False, label:str, **kwargs: Any):
        print(label)

        start_time = time.time()
        return_value = func(*args, verbose=verbose, **kwargs)
        end_time = time.time()
        duration = end_time - start_time

        if verbose:
            print(f"{label} finished in {round(duration, 2)}s")
        else:
            move_cursor_up_one_line = "\x1b[1A"
            print(f'{move_cursor_up_one_line}{label} {dim(f"{round(duration, 2)}s")}')

        return return_value
    return inside
