from kybra import query, opt, text


@query
def simple_query() -> opt[text]:
    return 'This is a query function'
