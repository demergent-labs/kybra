from kybra import blob, float32, float64, ic, int8, int16, int32, int64, nat, nat8, nat16, nat32, nat64, null, opt, Principal, query, Record, reserved, text, Variant
from typing import TypeAlias

MyTypeAlias: TypeAlias = opt[blob]
MyAlias = list[int16]


class SimpleRecord(Record):
    from_: str


class KeywordRecord(Record):
    False_: bool
    True_: str
    and_: int
    assert_: int8
    class_: int16
    def_: int32
    del_: int64
    elif_: nat
    except_: nat8
    finally_: nat16
    from_: nat32
    global_: nat64
    import_: float32
    is_: float64
    lambda_: opt[str]
    nonlocal_: text
    not_: SimpleRecord
    or_: 'WithRecord'
    pass_: 'KeywordVariant'
    raise_: blob
    with_: Principal
    # # The below python keywords are also
    None_: tuple[bool, bool]
    as_: null
    async_: reserved
    await_: list[nat]
    break_: MyTypeAlias
    continue_: MyAlias
    else_: opt[bool]
    for_: bool
    if_: bool
    in_: bool
    return_: bool
    try_: bool
    while_: bool
    yield_: bool


class WithRecord(Record):
    with__: bool
    with___: bool
    with____: bool
    with________________________________________________________________________: bool
    with______1: bool


class KeywordVariant(Variant, total=False):
    False_: null
    True_: null
    and_: null
    assert_: null
    class_: null
    def_: null
    del_: null
    elif_: null
    except_: null
    finally_: null
    from_: null
    global_: null
    import_: null
    is_: null
    lambda_: null
    nonlocal_: null
    not_: null
    or_: null
    pass_: null
    raise_: null
    with_: null


class RustKeywordRecord(Record):
    abstract: bool
    become: str
    const: int
    crate: nat
    fn: text
    mut: reserved
    pub: null
    type: blob


class RustKeywordVariant(Variant, total=False):
    type: null
    pub: null
    fn: null


@query
def keyword_variant(variant_keyword: KeywordVariant) -> KeywordVariant:
    ic.print(variant_keyword)
    return variant_keyword


@query
def simple_keyword(simple_record: SimpleRecord) -> SimpleRecord:
    ic.print(simple_record["from_"])
    return simple_record


@query
def rust_keyword() -> RustKeywordRecord:
    return {
        'abstract': False,
        'become': 'Become',
        'const': 3,
        'crate': 7,
        'fn': 'Function',
        'mut': False,
        'pub': None,
        'type': 'Type'.encode('utf-8')
    }


@query
def rust_keyword_variant() -> RustKeywordVariant:
    return {'type': None}


@query
def complex_keyword() -> KeywordRecord:
    return {
        'False_': False,
        'True_': 'False',
        'and_': 1,
        'assert_': 2,
        'class_': 3,
        'def_': 4,
        'del_': 5,
        'elif_': 6,
        'except_': 7,
        'finally_': 8,
        'from_': 9,
        'global_': 10,
        'import_': 11.12,
        'is_': 13.14,
        'lambda_': 'False',
        'nonlocal_': 'False',
        'not_': {'from_': 'False'},
        'or_': {
            'with__': False,
            'with___': False,
            'with____': False,
            'with________________________________________________________________________': False,
            'with______1': False
        },
        'pass_': {'raise_': None},
        'raise_': 'false'.encode('utf-8'),
        'with_': Principal.from_str('aaaaa-aa'),
        'None_': (False, True),
        'as_': None,
        'async_': False,
        'await_': [18, 19, 20],
        'break_': None,
        'continue_': [21, 22, 23],
        'else_': False,
        'for_': False,
        'if_': False,
        'in_': False,
        'return_': False,
        'try_': False,
        'while_': False,
        'yield_': False,
    }
