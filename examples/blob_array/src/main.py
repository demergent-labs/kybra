from kybra import blob, query

@query
def get_blob() -> blob:
    return string_to_blob('hello')

@query
def get_blobs() -> list[blob]:
    return [string_to_blob('hello'), string_to_blob('world')]

def string_to_blob(string: str) -> blob:
    return string.encode('utf-8')
