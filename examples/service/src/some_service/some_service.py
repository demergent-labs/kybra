from kybra import query, update


@query
def query1() -> bool:
    return True


@update
def update1() -> str:
    return 'SomeService update1'
