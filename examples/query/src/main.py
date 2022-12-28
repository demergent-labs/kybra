from kybra import query, opt, text, Record
import os as cool_os
from sys import argv as cool_argv
import os.path
# import kybra as kybra2


class Thing(Record):
    other: bool


class MyClass:
    global_var = "global variable"

    def __init__(self):
        self.instance_var = "instance variable"

    def first_function(self):
        print("This is the first function")

    def second_function(self):
        print("This is the second function")


print("'Tis the season")

# TODO elif is problematic because it counts the same as a nested if statement

raise ValueError("This is a value I don't like")


# Use a literal to specify a value
10

# Use a variable to refer to a value
x = 10

# Use an operator to perform a calculation
5 + 7

# Use a function call to invoke a function
len([1, 2, 3])

# Use an indexing expression to access an element of a sequence
[1, 2, 3][1]

# Use an attribute reference to access an attribute of an object
"hello".upper()

# Use a slicing expression to access a slice of a sequence
[1, 2, 3][1:3]

# Use a membership expression to test membership in a sequence
3 in [1, 2, 3]

# Use a comparison expression to compare two values
5 < 7

# Use a logical expression to combine multiple conditions
(x > 0) and (x < 10)

# Use a conditional expression to choose a value based on a condition
"positive" if x > 0 else "negative"

# Use a generator expression to create a generator
(i ** 2 for i in range(10))

# Use a list comprehension to create a list
[i ** 2 for i in range(10)]

# Use a set comprehension to create a set
{i ** 2 for i in range(10)}

# Use a dictionary comprehension to create a dictionary
{i: i ** 2 for i in range(10)}

# Use an exception handler to handle an exception
try:
    x = 10 / 0
except ValueError("Thing"):
    print("division by zero")

# Use a yield expression to produce a value in a generator function


def infinite_sequence():
    i = 0
    while True:
        yield i
        i += 1


# from kybra import thing
# print(thing)

# thing = 3

# TODO Match don't work
# match 3:
#     case 1:
#         print()
#     case 2:
#         print()
#     case 3:
#         print()
#     case _:
#         print()
def my_function(arg1):
    print("arg1:", arg1)  # Here is a comment
    print("args:", args)
    print("keyword_arg:", keyword_arg)
    print("kwargs:", kwargs)


def my_function_2(arg1):
    print("arg1:", arg1)  # Here is a comment


def my_function_2_25(arg1):
    # match 3:
    #     case 1:
    #         print()
    #     case 2:
    #         print()
    #     case 3:
    #         print()
    #     case _:
    #         print()
    print("arg1:", arg1)  # Here is a comment
    print(
















    )


def my_function_2_5():
    return print


()


def my_function_3(arg1, *args):
    print("arg1:", arg1)
    print("args:", args)
    print("keyword_arg:", keyword_arg)
    print("kwargs:", kwargs)


def my_function_4(arg1, *args, keyword_arg=None):
    print("arg1:", arg1)
    print("args:", args)
    print("keyword_arg:", keyword_arg)
    print("kwargs:", kwargs)


def my_function_5(arg1, *args, keyword_arg=None, **kwargs):
    print("arg1:", arg1)
    print("args:", args)
    print("keyword_arg:", keyword_arg)
    print("kwargs:", kwargs)


def thingy():
    # This is a comment
    print(
        {
            'thing': 1
        }
    )


@query
async def simple_query() -> opt[text]:
    lambda x: print(x)
    try:
        x = 10 / 0
    except ZeroDivisionError:
        print("division by zero")
    try:
        x = 10 / 0
    except ZeroDivisionError:
        try:
            raise ValueError("division by zero") from TypeError("invalid type")
        except ValueError:
            try:
                raise
            except ValueError as e:
                print("caught ValueError:", e)
                print("cause:", e.__cause__)
            finally:
                print("finally 1")
        except TypeError as e:
            print("caught TypeError:", e)
        finally:
            print("finally 2")
    except Exception as e:
        print("caught Exception:", e)
    finally:
        print("finally 3")
    raise
    raise ValueError("invalid value") from TypeError("invalid type")
    with open("file.text", "w") as f:
        f.write('Hello')
    global thing
    thing = [1, 2, 3, 4, 5]
    [x for x in thing if x == 1]
    numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    even_squares = [n**2 for n in numbers if n % 2 == 0 if n % 3 == 0]
    even_odd = ["even" if n % 2 == 0 else "odd" for n in numbers]
    print(even_odd)
    print(even_squares)
    if 1 is not 2:
        print("Hello' there")
        print("Hello there")
        print("Hello there")
    else:
        if 1 not in [1]:
            print("hmmm")
            print("Hello there")
        else:
            print("Hello there")
    while True:
        print("hello there")
        pass
        continue
    else:
        print("hello world")
    async for index in range(5):
        print(index)
    else:
        print("Thats a wrap")
    thing = {"hello": 1}
    del thing['hello']
    return 'This is a simple query'


# thing = 1

# '''
#     # !@#$!@#$!@#$$#%^$^&^%&*^&^()*()&(^*&$%^&#$#%#@)
#     thing = 1 ** 2
#     thing = [[1]] @ [[2]]
#     thing = 1 + 2
#     thing = 1 ^ 2
#     thing = 1 - 1
#     thing = 1 * 1
#     thing = 1 / 1
#     thing = 1 % 1
#     thing = 1 << 1
#     thing = 1 >> 1
#     thing = 1 | 1
#     thing = 1 & 1
#     print(thing)
# '''

# TODO the "hello".upper() example is bleeding over into the next one as well
# TODO The canister results with the ellipsis aren't quite right
# TODO The IC object is also not working.
# TODO The AsyncInfo class is also not working.
# TODO The Principal class is also not working.
# TODO The AccountIdentifier class is also not working.
# TODO whatever the statement on custom_modules/principal.py on line 160-163 is not working
# TODO b32encode is not working
# TODO I am wondering if the """ Style strings are messing it up? because b32decode on line 203 is bleeding over into the next function
