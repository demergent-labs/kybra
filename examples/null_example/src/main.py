from kybra import ic, null, query, Record, update, void


class PartiallyNullRecord(Record):
    first_item: int
    second_item: null
    third_item: int


class TwoNullRecord(Record):
    first_item: null
    second_item: null


class ThreeNullRecord(Record):
    first_item: null
    second_item: null
    third_item: null


@query
def null_function(param: null) -> null:
    return param


@query
def void_is_not_null() -> void:
    ic.print(
        "Even though they are both None in Python, for Candid null and void are different.")


@query
def get_partially_null_record() -> PartiallyNullRecord:
    return {
        'first_item': 1,
        'second_item': None,
        'third_item': 3
    }


@update
def set_partially_null_record(param: PartiallyNullRecord) -> PartiallyNullRecord:
    return param


@query
def get_small_null_record() -> TwoNullRecord:
    return {
        'first_item': None,
        'second_item': None
    }


@update
def set_small_null_record(param: TwoNullRecord) -> TwoNullRecord:
    return param


@query
def get_large_null_record() -> ThreeNullRecord:
    return {
        'first_item': None,
        'second_item': None,
        'third_item': None
    }


@update
def set_large_null_record(param: ThreeNullRecord) -> ThreeNullRecord:
    return param
