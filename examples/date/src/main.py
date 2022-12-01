# TODO Update this example to match the Azle date example more closely once we can support datetime

from kybra import float64, query

from time import gmtime, strftime, time

@query
def get_time() -> float64:
    return time()

@query
def get_strftime() -> str:
    return strftime("%a %b %d %Y", gmtime())
