from kybra import opt, query, Record

class Element(Record):
    id: str

class Head(Record):
    elements: list[Element]

class Html(Record):
    head: opt[Head]

@query
def get_html() -> Html:
    return {
        'head': None
    }

@query
def get_head() -> opt[Head]:
    return {
        'elements': []
    }

@query
def get_head_with_elements() -> opt[Head]:
    return {
        'elements': [
            {
                'id': '0'
            }
        ]
    }

@query
def get_element(element: opt[opt[Element]]) -> opt[opt[Element]]:
    return element
