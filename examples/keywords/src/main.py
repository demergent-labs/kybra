from kybra import ic, query, Record, Variant

class SimpleRecord(Record):
    from_: str

class KeywordRecord(Record):
    False_: bool
    True_: bool
    and_: bool
    assert_: bool
    class_: bool
    def_: bool
    del_: bool
    elif_: bool
    except_: bool
    finally_: bool
    from_: bool
    global_: bool
    import_: bool
    is_: bool
    lambda_: bool
    nonlocal_: bool
    not_: bool
    or_: bool
    pass_: bool
    raise_: bool
    with_: bool
    # The below python keywords are also
    None_: bool
    as_: bool
    async_: bool
    await_: bool
    break_: bool
    continue_: bool
    else_: bool
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

class KeywordVariant(Variant):
    False_: None
    True_: None
    and_: None
    assert_: None
    class_: None
    def_: None
    del_: None
    elif_: None
    except_: None
    finally_: None
    from_: None
    global_: None
    import_: None
    is_: None
    lambda_: None
    nonlocal_: None
    not_: None
    or_: None
    pass_: None
    raise_: None
    with_: None

@query
def keyword_method(record_keyword: KeywordRecord, variant_keyword: KeywordVariant, with_keyword: WithRecord) -> tuple[KeywordRecord, KeywordVariant, WithRecord]:
    return (record_keyword, variant_keyword, with_keyword)

@query
def simple_keyword(simple_record: SimpleRecord) -> SimpleRecord:
    ic.print(simple_record["from_"])
    return simple_record

@query
def complex_keyword() -> KeywordRecord:
    return {
        'False_': False,
        'True_': False,
        'and_': False,
        'assert_': False,
        'class_': False,
        'def_': False,
        'del_': False,
        'elif_': False,
        'except_': False,
        'finally_': False,
        'from_': False,
        'global_': False,
        'import_': False,
        'is_': False,
        'lambda_': False,
        'nonlocal_': False,
        'not_': False,
        'or_': False,
        'pass_': False,
        'raise_': False,
        'with_': False,
        'None_': False,
        'as_': False,
        'async_': False,
        'await_': False,
        'break_': False,
        'continue_': False,
        'else_': False,
        'for_': False,
        'if_': False,
        'in_': False,
        'return_': False,
        'try_': False,
        'while_': False,
        'yield_': False,
    }
