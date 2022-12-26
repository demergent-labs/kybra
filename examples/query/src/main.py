# from kybra import query, opt, text


@query
def simple_query() -> opt[text]:
    return 'This is a query function'


'''

    thing = 1 ** 2
    thing = [[1]] @ [[2]]
    thing = 1 + 2
    thing = 1 ^ 2
    thing = 1 - 1
    thing = 1 * 1
    thing = 1 / 1
    thing = 1 % 1
    thing = 1 << 1
    thing = 1 >> 1
    thing = 1 | 1
    thing = 1 & 1
    print(thing)
'''
