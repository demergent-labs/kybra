# from kybra import query, opt, text, Record
# import os as cool_os
# from sys import argv as cool_argv
# import os.path
# # import kybra as kybra2


# class Thing(Record):
#     other: bool


# class MyClass:
#     global_var = "global variable"

#     def __init__(self):
#         self.instance_var = "instance variable"

#     def first_function(self):
#         print("This is the first function")

#     def second_function(self):
#         print("This is the second function")


# print("'Tis the season")

# # TODO elif is problematic because it counts the same as a nested if statement

# raise ValueError("This is a value I don't like")


# # Use a literal to specify a value
# 10

# # Use a variable to refer to a value
# x = 10

# # Use an operator to perform a calculation
# 5 + 7

# # Use a function call to invoke a function
# len([1, 2, 3])

# # Use an indexing expression to access an element of a sequence
# [1, 2, 3][1]

# TODO this is going to be more challenging and I'll come back to it later.
# # Use an attribute reference to access an attribute of an object
# "hello".upper()

# # Do the same thing but assign the value to a variable first
# thing = "hello"
# thing.lower()

# # Do the same thing but with a different type of constant
# (3).real

# # Put it in a function to see if it does it differently
# def hello_upper() -> str:
#     "hello".lower()
#     return "hello".upper()


# # Use a slicing expression to access a slice of a sequence
# [1, 2, 3][1:3]

# # Use a membership expression to test membership in a sequence
# 3 in [1, 2, 3]

# # Use a comparison expression to compare two values
# 5 < 7

# # Use a logical expression to combine multiple conditions
# (x > 0) and (x < 10)

# # Use a conditional expression to choose a value based on a condition
# "positive" if x > 0 else "negative"

# # Use a generator expression to create a generator
# (i ** 2 for i in range(10))

# # Use a list comprehension to create a list
# [i ** 2 for i in range(10)]

# # Use a set comprehension to create a set
# {i ** 2 for i in range(10)}

# # Use a dictionary comprehension to create a dictionary
# {i: i ** 2 for i in range(10)}

# # Use an exception handler to handle an exception
# try:
#     x = 10 / 0
# except ValueError("Thing"):
#     print("division by zero")

# # Use a yield expression to produce a value in a generator function


# def infinite_sequence():
#     i = 0
#     while True:
#         yield i
#         i += 1


# # from kybra import thing
# # print(thing)

# # thing = 3

# # TODO Match don't work
# # match 3:
# #     case 1:
# #         print()
# #     case 2:
# #         print()
# #     case 3:
# #         print()
# #     case _:
# #         print()
# def my_function(arg1):
#     print("arg1:", arg1)  # Here is a comment
#     print("args:", args)
#     print("keyword_arg:", keyword_arg)
#     print("kwargs:", kwargs)


# def my_function_2(arg1):
#     print("arg1:", arg1)  # Here is a comment

# def my_function_2_125(arg1: int, arg2: str, arg3: bool) -> bool:
#     print("arg1:", arg1)  # Here is a comment


# def call_raw(canister_id: Principal, method: str, args_raw: blob, payment: nat64) -> CanisterResult[T]:
#     print(canister_id)

# def my_function_2_25(arg1):
#     # match 3:
#     #     case 1:
#     #         print()
#     #     case 2:
#     #         print()
#     #     case 3:
#     #         print()
#     #     case _:
#     #         print()
#     print("arg1:", arg1)  # Here is a comment
#     print(


#     )


# def my_function_2_5():
#     return print


# ()


# def my_function_3(arg1, *args):
#     print("arg1:", arg1)
#     print("args:", args)
#     print("keyword_arg:", keyword_arg)
#     print("kwargs:", kwargs)


# def my_function_4(arg1, *args, keyword_arg=None):
#     print("arg1:", arg1)
#     print("args:", args)
#     print("keyword_arg:", keyword_arg)
#     print("kwargs:", kwargs)


# def my_function_5(arg1, *args, keyword_arg=None, **kwargs):
#     print("arg1:", arg1)
#     print("args:", args)
#     print("keyword_arg:", keyword_arg)
#     print("kwargs:", kwargs)


# def thingy():
#     # This is a comment
#     print(
#         {
#             'thing': 1
#         }
#     )


# @query
# async def simple_query() -> opt[text]:
#     lambda x: print(x)
#     try:
#         x = 10 / 0
#     except ZeroDivisionError:
#         print("division by zero")
#     try:
#         x = 10 / 0
#     except ZeroDivisionError:
#         try:
#             raise ValueError("division by zero") from TypeError("invalid type")
#         except ValueError:
#             try:
#                 raise
#             except ValueError as e:
#                 print("caught ValueError:", e)
#                 print("cause:", e.__cause__)
#             finally:
#                 print("finally 1")
#         except TypeError as e:
#             print("caught TypeError:", e)
#         finally:
#             print("finally 2")
#     except Exception as e:
#         print("caught Exception:", e)
#     finally:
#         print("finally 3")
#     raise
#     raise ValueError("invalid value") from TypeError("invalid type")
#     with open("file.text", "w") as f:
#         f.write('Hello')
#     global thing
#     thing = [1, 2, 3, 4, 5]
#     [x for x in thing if x == 1]
#     numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
#     even_squares = [n**2 for n in numbers if n % 2 == 0 if n % 3 == 0]
#     even_odd = ["even" if n % 2 == 0 else "odd" for n in numbers]
#     print(even_odd)
#     print(even_squares)
#     if 1 is not 2:
#         print("Hello' there")
#         print("Hello there")
#         print("Hello there")
#     else:
#         if 1 not in [1]:
#             print("hmmm")
#             print("Hello there")
#         else:
#             print("Hello there")
#     while True:
#         print("hello there")
#         pass
#         continue
#     else:
#         print("hello world")
#     async for index in range(5):
#         print(index)
#     else:
#         print("Thats a wrap")
#     thing = {"hello": 1}
#     del thing['hello']
#     return 'This is a simple query'


# # thing = 1

# # '''
# #     # !@#$!@#$!@#$$#%^$^&^%&*^&^()*()&(^*&$%^&#$#%#@)
# #     thing = 1 ** 2
# #     thing = [[1]] @ [[2]]
# #     thing = 1 + 2
# #     thing = 1 ^ 2
# #     thing = 1 - 1
# #     thing = 1 * 1
# #     thing = 1 / 1
# #     thing = 1 % 1
# #     thing = 1 << 1
# #     thing = 1 >> 1
# #     thing = 1 | 1
# #     thing = 1 & 1
# #     print(thing)
# # '''

# # TODO The IC object is also not working.


# # TODO The canister results with the ellipsis aren't quite right
# # Handle the ellipsis
# class CanisterResult(Generic[T]):
#     ok: T
#     err: Optional[str]

#     def __init__(self, ok: T, err: str):
#         self.ok = ok
#         self.err = err

#     def notify(self) -> "NotifyResult": ...

#     def with_cycles(self, cycles: nat64) -> "CanisterResult[T]": ...

#     def with_cycles128(self, cycles: nat) -> "CanisterResult[T]": ...

# # Make sure calls that are part of attributes work
# class c1():
#     def a():
#         return ic123.k()

#     def b():
#         a()

#     def call_raw(canister_id: Principal, method: str, args_raw: blob, payment: nat64) -> CanisterResult[T]:
#         return None


# # Make sure decorators work
# class c2():
#     @staticmethod
#     def a():
#         return ic123.k()

#     @staticmethod
#     def b():
#         a()


# Make sure that f strings work
# class c3():
#     def function_with_f_string(self, person: str) -> str:
#         return f'Hello {person}'


# # bstring statement
# # TODO
# _b32alphabet = b'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567'


# # bstring and fstring if a function
# # TODO
# def function_with_f_and_b_string():
#     bstring = b'==='
#     fstring = f'Hello {bstring}'
#     print(fstring)
