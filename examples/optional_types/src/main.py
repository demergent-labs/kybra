from kybra import Opt, query, Record, Vec


class Element(Record):
    id: str


class Head(Record):
    elements: Vec[Element]


class Html(Record):
    head: Opt[Head]


@query
def get_html() -> Html:
    return {"head": None}


@query
def get_head() -> Opt[Head]:
    return {"elements": []}


@query
def get_head_with_elements() -> Opt[Head]:
    return {"elements": [{"id": "0"}]}


@query
def get_element(element: Opt[Opt[Element]]) -> Opt[Opt[Element]]:
    return element
