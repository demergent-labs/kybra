from kybra import query, Record, Variant, canister, Query, Update, Oneway, Func
from typing import TypeAlias

MyFunc: TypeAlias = Func(Query[[], str])

class User(Record):
    id: str
    username: str

class Skill(Variant, total=False):
    Carpentry: None
    Programming: None
    Running: int

# @query
# def get_user() -> User:
#     # the_tuple = my_func.get_tuple()

#     # TODO trying to figure out types for func
#     # principal = my_func.get_principal()
#     # method_name = my_func.get_method_name()

#     # principal = my_func[0]
#     # method_name = my_func[1]

#     return {
#         'id': '0',
#         'username': 'lastmjs'
#     }

# @query
# def get_skill_0() -> Skill:
#     return {
#         'Carpentry': None
#     }

# @query
# def get_skill_1() -> Skill:
#     return {
#         'Running': 26
#     }

@query
def test() -> str:
    return 'it works'
