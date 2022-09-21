# TODO should go in an azle.py file that can be imported
# from typing import Any # TODO importing Any seems to break in many cases (maybe all cases?)

int64 = int
int32 = int
int16 = int
int8 = int

nat = int
nat64 = int
nat32 = int
nat16 = int
nat8 = int

float32 = float
float64 = float

text = str

# blob = bytes

# reserved = Any
# empty = Any

def query(func: object):
    return func

def update(func: object):
    return func
# TODO should go in an azle.py file that can be imported

@query
def monkey(x: str, y: str) -> str:
    return x + y
