import base64
import collections
import datetime
import itertools
import json
import random
import string
import urllib.parse
import uuid

from kybra import blob, float64, query, update, Vec

@query
def test_base64() -> blob:
    return base64.b64encode(b'Hello there sir')

@query
def test_collections() -> str:
    counter = collections.Counter(["apple", "banana", "orange", "apple", "banana", "apple"])
    return counter.most_common(1)[0][0]

@query
def test_datetime() -> str:
    return str(datetime.datetime.now())

@query
def test_itertools() -> Vec[str]:
    perms = itertools.permutations("abcd", 2)
    result = ["".join(p) for p in perms]
    return result

@query
def test_json() -> str:
    return json.dumps({
        "hello": "world"
    })

@update
def test_random() -> float64:
    return random.random()

@query
def test_string() -> str:
    return string.ascii_letters

@query
def test_urllib() -> str:
    return urllib.parse.unquote("https%3A%2F%2Fwww.example.com%2Fsearch%3Fquery%3Dtest%26page%3D1")

@update
def test_uuid() -> str:
    return str(uuid.uuid4())