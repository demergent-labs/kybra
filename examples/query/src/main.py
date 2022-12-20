from kybra import query, text


@query
def simple_query() -> text:
    return 'This is a query function'
