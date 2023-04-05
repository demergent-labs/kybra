from kybra import (
    blob,
    float32,
    ic,
    int8,
    manual,
    nat,
    nat8,
    null,
    Record,
    query,
    reserved,
    update,
    Variant
)


class Options(Variant, total=False):
    Small: null
    Medium: null
    Large: null


class RawReply(Record):
    int: int
    text: str
    bool: bool
    blob: blob
    variant: Options


class Element(Record):
    id: str
    orbitals: list["Orbital"]
    state: "State"


class Orbital(Record):
    layer: nat8
    electrons: nat8


class State(Variant, total=False):
    Gas: "Gas"
    Liquid: null
    Solid: "Solid"


class Solid(Record):
    element: str


class Gas(Variant, total=False):
    Elemental: null
    Mixed: null
    Toxic: null

# Updates


@update
def manual_update(message: str) -> manual[str]:
    if (message == 'reject'):
        ic.reject(message)
        return

    ic.reply(message)


@update
def update_blob() -> manual[blob]:
    ic.reply(bytes([83, 117, 114, 112, 114, 105, 115, 101, 33]))


@update
def update_float32() -> manual[float32]:
    ic.reply(1245.678)


@update
def update_inline_type() -> manual[tuple[str, str]]:
    ic.reply(('Hello', 'World'))


@update
def update_int8() -> manual[int8]:
    ic.reply(-100)


@update
def update_nat() -> manual[nat]:
    ic.reply(184_467_440_737_095_516_150)


@update
def update_null() -> manual[null]:
    ic.reply(None)


@update
def update_record() -> manual[Element]:
    element: Element = {
        'id': 'b0283eb7-9c0e-41e5-8089-3345e6a8fa6a',
        'orbitals': [
            {
                'electrons': 2,
                'layer': 1
            },
            {
                'electrons': 8,
                'layer': 2
            }
        ],
        'state': {
            'Gas': {'Elemental': None}
        }
    }
    ic.reply(element)


@update
def update_reserved() -> manual[reserved]:
    ic.reply('anything')


@update
def update_string() -> manual[str]:
    ic.reply('hello')


@update
def update_variant() -> manual[Gas]:
    gas: Gas = {'Toxic': None}
    ic.reply(gas)


# Queries

@query
def manual_query(message: str) -> manual[str]:
    if (message == 'reject'):
        ic.reject(message)
        return

    ic.reply(message)


@query
def query_blob() -> manual[blob]:
    ic.reply(bytes([83, 117, 114, 112, 114, 105, 115, 101, 33]))


@query
def query_float32() -> manual[float32]:
    ic.reply(1245.678)


@query
def query_int8() -> manual[int8]:
    ic.reply(-100)


@query
def query_nat() -> manual[nat]:
    ic.reply(184467440737095516150)


@query
def query_null() -> manual[null]:
    ic.reply(None)


@query
def query_record() -> manual[Element]:
    element: Element = {
        'id': 'b0283eb7-9c0e-41e5-8089-3345e6a8fa6a',
        'orbitals': [
            {
                'electrons': 2,
                'layer': 1
            },
            {
                'electrons': 8,
                'layer': 2
            }
        ],
        'state': {
            'Gas': {'Elemental': None}
        }
    }
    ic.reply(element)


@query
def query_reserved() -> manual[reserved]:
    ic.reply("anything")


@query
def query_string() -> manual[str]:
    ic.reply('hello')


@query
def query_variant() -> manual[Gas]:
    gas = {'Toxic': None}
    ic.reply(gas)


@query
def reply_raw() -> manual[RawReply]:
    ic.reply_raw(
        ic.candid_encode(
            '(record { "int" = 42; "text" = "text"; "bool" = true; "blob" = blob "Surprise!"; "variant" = variant { Medium } })'
        )
    )
