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

# # Use an attribute reference to access an attribute of an object
# "hello".upper()

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

# # TODO the "hello".upper() example is bleeding over into the next one as well
# # TODO The canister results with the ellipsis aren't quite right
# # TODO The IC object is also not working.
# # TODO The AsyncInfo class is also not working.
# # TODO The Principal class is also not working.
# # TODO The AccountIdentifier class is also not working.
# # TODO whatever the statement on custom_modules/principal.py on line 160-163 is not working
# # TODO b32encode is not working
# # TODO I am wondering if the """ Style strings are messing it up? because b32decode on line 203 is bleeding over into the next function

# TODO I think that this is caused by the token starting at h and then running into the other quote?
# Use an attribute reference to access an attribute of an object
"hello".upper()

# Use a slicing expression to access a slice of a sequence
[1, 2, 3][1:3]


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


# # Do we have a problem with static methods?
# class ic(Generic[T]):
#     @staticmethod
#     def accept_message():
#         _kybra_ic._kybra_accept_message()  # type: ignore

#     @staticmethod
#     def arg_data_raw() -> blob:
#         return _kybra_ic._kybra_arg_data_raw()  # type: ignore

#     @staticmethod
#     def arg_data_raw_size() -> nat32:
#         return _kybra_ic._kybra_arg_data_raw_size()  # type: ignore

#     @staticmethod
#     def call_raw(canister_id: Principal, method: str, args_raw: blob, payment: nat64) -> CanisterResult[T]:
#         return AsyncInfo('call_raw', [
#             canister_id,
#             method,
#             args_raw,
#             payment
#         ])  # type: ignore

#     @staticmethod
#     def call_raw128(canister_id: Principal, method: str, args_raw: blob, payment: nat) -> CanisterResult[T]:
#         return AsyncInfo('call_raw128', [
#             canister_id,
#             method,
#             args_raw,
#             payment
#         ])  # type: ignore

#     @staticmethod
#     def caller() -> Principal:
#         return _kybra_ic._kybra_caller()  # type: ignore

#     @staticmethod
#     def candid_encode(candid_string: str) -> blob:
#         return _kybra_ic._kybra_candid_encode(candid_string)  # type: ignore

#     @staticmethod
#     def candid_decode(candid_encoded: blob) -> str:
#         return _kybra_ic._kybra_candid_decode(candid_encoded)  # type: ignore

#     @staticmethod
#     def canister_balance() -> nat64:
#         return _kybra_ic._kybra_canister_balance()  # type: ignore

#     @staticmethod
#     def canister_balance128() -> nat:
#         return _kybra_ic._kybra_canister_balance128()  # type: ignore

#     @staticmethod
#     def clear_timer(id: TimerId) -> None:
#         return _kybra_ic._kybra_clear_timer(id)  # type: ignore

#     @staticmethod
#     def data_certificate() -> opt[blob]:
#         return _kybra_ic._kybra_data_certificate()  # type: ignore

#     @staticmethod
#     def id() -> Principal:
#         return _kybra_ic._kybra_id()  # type:ignore

#     @staticmethod
#     def method_name() -> str:
#         return _kybra_ic._kybra_method_name()  # type:ignore

#     @staticmethod
#     def msg_cycles_accept(max_amount: nat64) -> nat64:
#         return _kybra_ic._kybra_msg_cycles_accept(max_amount)  # type: ignore

#     @staticmethod
#     def msg_cycles_accept128(max_amount: nat) -> nat:
#         # type: ignore
#         return _kybra_ic._kybra_msg_cycles_accept128(max_amount)

#     @staticmethod
#     def msg_cycles_available() -> nat64:
#         return _kybra_ic._kybra_msg_cycles_available()  # type: ignore

#     @staticmethod
#     def msg_cycles_available128() -> nat:
#         return _kybra_ic._kybra_msg_cycles_available128()  # type: ignore

#     @staticmethod
#     def msg_cycles_refunded() -> nat64:
#         return _kybra_ic._kybra_msg_cycles_refunded()  # type: ignore

#     @staticmethod
#     def msg_cycles_refunded128() -> nat:
#         return _kybra_ic._kybra_msg_cycles_refunded128()  # type: ignore

#     @staticmethod
#     def notify_raw(
#         canister_id: Principal,
#         method: str,
#         args_raw: blob,
#         payment: nat
#     ) -> NotifyResult:
#         # type: ignore
#         return _kybra_ic._kybra_notify_raw(canister_id, method, args_raw, payment)

#     @staticmethod
#     def performance_counter(counter_type: nat32) -> nat64:
#         # type: ignore
#         return _kybra_ic._kybra_performance_counter(counter_type)

#     @staticmethod
#     def print(x: Any):
#         _kybra_ic._kybra_print(str(x))  # type: ignore

#     @staticmethod
#     def reject(x: Any):
#         _kybra_ic._kybra_reject(x)  # type: ignore

#     @staticmethod
#     def reject_code() -> RejectionCode:
#         return _kybra_ic._kybra_reject_code()  # type: ignore

#     @staticmethod
#     def reject_message() -> str:
#         return _kybra_ic._kybra_reject_message()  # type: ignore

#     @staticmethod
#     def reply(value: Any):
#         first_called_function_name = get_first_called_function_name()
#         _kybra_ic._kybra_reply(
#             first_called_function_name, value)  # type: ignore

#     @staticmethod
#     def reply_raw(x: Any):
#         _kybra_ic._kybra_reply_raw(x)  # type: ignore

#     @staticmethod
#     def set_certified_data(data: blob):
#         _kybra_ic._kybra_set_certified_data(data)  # type: ignore

#     @staticmethod
#     def set_timer(delay: Duration, func: Callable[[], None]) -> TimerId:
#         return _kybra_ic._kybra_set_timer(delay, func)  # type: ignore

#     @staticmethod
#     def set_timer_interval(interval: Duration, func: Callable[[], None]) -> TimerId:
#         # type: ignore
#         return _kybra_ic._kybra_set_timer_interval(interval, func)

#     @staticmethod
#     def stable_bytes() -> blob:
#         return _kybra_ic._kybra_stable_bytes()  # type: ignore

#     @staticmethod
#     def stable_grow(new_pages: nat32) -> StableGrowResult:
#         return _kybra_ic._kybra_stable_grow(new_pages)  # type: ignore

#     @staticmethod
#     def stable_read(offset: nat32, length: nat32) -> blob:
#         return _kybra_ic._kybra_stable_read(offset, length)  # type: ignore

#     @staticmethod
#     def stable_size() -> nat32:
#         return _kybra_ic._kybra_stable_size()  # type: ignore

#     @staticmethod
#     def stable_write(offset: nat32, buf: blob):
#         _kybra_ic._kybra_stable_write(offset, buf)  # type: ignore

#     @staticmethod
#     def stable64_grow(new_pages: nat64) -> Stable64GrowResult:
#         return _kybra_ic._kybra_stable64_grow(new_pages)  # type: ignore

#     @staticmethod
#     def stable64_read(offset: nat64, length: nat64) -> blob:
#         return _kybra_ic._kybra_stable64_read(offset, length)  # type: ignore

#     @staticmethod
#     def stable64_size() -> nat64:
#         return _kybra_ic._kybra_stable64_size()  # type: ignore

#     @staticmethod
#     def stable64_write(offset: nat64, buf: blob):
#         _kybra_ic._kybra_stable64_write(offset, buf)  # type: ignore

#     @staticmethod
#     def stable_storage() -> Any:
#         return _kybra_stable_storage  # type: ignore

#     @staticmethod
#     def time() -> nat64:
#         return _kybra_ic._kybra_time()  # type: ignore

#     @staticmethod
#     def trap(message: str) -> NoReturn:  # type: ignore
#         _kybra_ic._kybra_trap(message)  # type: ignore


# # What is up with this?
# class AsyncInfo:
#     name: str
#     args: list[Any]

#     def __init__(self, name: str, args: list[Any]):
#         self.name = name
#         self.args = args

#     def with_cycles(self, cycles: nat64) -> "AsyncInfo":
#         return AsyncInfo('call_with_payment', [*self.args, cycles])

#     def with_cycles128(self, cycles: nat) -> "AsyncInfo":
#         return AsyncInfo('call_with_payment128', [*self.args, cycles])

#     def notify(self) -> NotifyResult:
#         qualname: str = self.args[1]
#         with_payment = 'with_payment128_' if self.name == 'call_with_payment' or self.name == 'call_with_payment128' else ''
#         notify_function_name = f'_kybra_notify_{with_payment}{qualname.replace(".", "_")}_wrapper'

#         # type: ignore
#         return getattr(_kybra_ic, notify_function_name)(self.args)


# # Another hint that maybe it's the decorators?
# class Principal:
#     def __init__(self, bytes: bytes = b''):
#         self._len = len(bytes)
#         self._bytes = bytes
#         self.hex = str(self._bytes.hex()).upper()
#         self._isPrincipal = True

#     @staticmethod
#     def management_canister():
#         return Principal()

#     @staticmethod
#     def self_authenticating(pubkey: Union[str, bytes]):
#         if isinstance(pubkey, str):
#             pubkey = bytes.fromhex(pubkey)
#         hash_ = hashlib.sha224(pubkey).digest()
#         hash_ += bytes([2])
#         return Principal(bytes=hash_)

#     @staticmethod
#     def anonymous():
#         return Principal(bytes=b'\x04')

#     @property
#     def len(self):
#         return self._len

#     @property
#     def bytes(self):
#         return self._bytes

#     @property
#     def isPrincipal(self):
#         return self._isPrincipal

#     @staticmethod
#     def from_str(s: str):
#         s1 = s.replace('-', '')
#         pad_len = math.ceil(len(s1) / 8) * 8 - len(s1)
#         b = b32decode(s1.upper().encode() + b'=' * pad_len)
#         if len(b) < CRC_LENGTH_IN_BYTES:
#             raise Exception("principal length error")
#         p = Principal(bytes=b[CRC_LENGTH_IN_BYTES:])
#         if not p.to_str() == s:
#             raise Exception("principal format error")
#         return p

#     @staticmethod
#     def from_hex(s: str):
#         return Principal(bytes.fromhex(s.lower()))

#     def to_str(self):
#         checksum = zlib.crc32(self._bytes) & 0xFFFFFFFF
#         b = b''
#         b += checksum.to_bytes(CRC_LENGTH_IN_BYTES, byteorder='big')
#         b += self.bytes
#         s = b32encode(b).decode('utf-8').lower().replace('=', '')
#         ret = ''
#         while len(s) > 5:
#             ret += s[:5]
#             ret += '-'
#             s = s[5:]
#         ret += s
#         return ret

#     def to_account_id(self, sub_account: int = 0):
#         return AccountIdentifier.new(self, sub_account)

#     def __repr__(self):
#         return "Principal(" + self.to_str() + ")"

#     def __str__(self):
#         return self.to_str()


# # Also with the decorators?
# class AccountIdentifier:
#     def __init__(self, hash: bytes) -> None:
#         assert len(hash) == 32
#         self._hash = hash

#     def to_str(self):
#         return '0x' + self._hash.hex()

#     def __repr__(self):
#         return "Account(" + self.to_str() + ")"

#     def __str__(self):
#         return self.to_str()

#     @property
#     def bytes(self) -> bytes:
#         return self._hash

#     @staticmethod
#     def new(principal: Principal, sub_account: int = 0):
#         sha224 = hashlib.sha224()
#         sha224.update(b'\x0Aaccount-id')
#         sha224.update(principal.bytes)

#         sub_account = sub_account.to_bytes(32, byteorder='big')
#         sha224.update(sub_account)
#         hash = sha224.digest()
#         checksum = zlib.crc32(hash) & 0xFFFFFFFF
#         account = checksum.to_bytes(
#             CRC_LENGTH_IN_BYTES, byteorder='big') + hash
#         return AccountIdentifier(account)


# # Good heavens what is this?
# # Base32 encoding/decoding must be done in Python
# _b32alphabet = b'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567'
# _b32tab2 = None
# _b32rev = None

# bytes_types = (bytes, bytearray)  # Types acceptable as binary data


# def b32encode(s):
#     """Encode the bytes-like object s using Base32 and return a bytes object.
#     """
#     global _b32tab2
#     # Delay the initialization of the table to not waste memory
#     # if the function is never called
#     if _b32tab2 is None:
#         b32tab = [bytes((i,)) for i in _b32alphabet]
#         _b32tab2 = [a + b for a in b32tab for b in b32tab]
#         b32tab = None

#     if not isinstance(s, bytes_types):
#         s = memoryview(s).tobytes()
#     leftover = len(s) % 5
#     # Pad the last quantum with zero bits if necessary
#     if leftover:
#         s = s + b'\0' * (5 - leftover)  # Don't use += !
#     encoded = bytearray()
#     from_bytes = int.from_bytes
#     b32tab2 = _b32tab2
#     for i in range(0, len(s), 5):
#         c = from_bytes(s[i: i + 5], 'big')
#         encoded += (b32tab2[c >> 30] +           # bits 1 - 10
#                     b32tab2[(c >> 20) & 0x3ff] + # bits 11 - 20
#                     b32tab2[(c >> 10) & 0x3ff] + # bits 21 - 30
#                     b32tab2[c & 0x3ff]           # bits 31 - 40
#                    )
#     # Adjust for any leftover partial quanta
#     if leftover == 1:
#         encoded[-6:] = b'======'
#     elif leftover == 2:
#         encoded[-4:] = b'===='
#     elif leftover == 3:
#         encoded[-3:] = b'==='
#     elif leftover == 4:
#         encoded[-1:] = b'='
#     return bytes(encoded)

# def b32decode(s, casefold=False, map01=None):
#     """Decode the Base32 encoded bytes-like object or ASCII string s.

#     Optional casefold is a flag specifying whether a lowercase alphabet is
#     acceptable as input.  For security purposes, the default is False.

#     RFC 3548 allows for optional mapping of the digit 0 (zero) to the
#     letter O (oh), and for optional mapping of the digit 1 (one) to
#     either the letter I (eye) or letter L (el).  The optional argument
#     map01 when not None, specifies which letter the digit 1 should be
#     mapped to (when map01 is not None, the digit 0 is always mapped to
#     the letter O).  For security purposes the default is None, so that
#     0 and 1 are not allowed in the input.

#     The result is returned as a bytes object.  A binascii.Error is raised if
#     the input is incorrectly padded or if there are non-alphabet
#     characters present in the input.
#     """
#     global _b32rev
#     # Delay the initialization of the table to not waste memory
#     # if the function is never called
#     if _b32rev is None:
#         _b32rev = {v: k for k, v in enumerate(_b32alphabet)}
#     s = _bytes_from_decode_data(s)
#     if len(s) % 8:
#         raise binascii.Error('Incorrect padding')
#     # Handle section 2.4 zero and one mapping.  The flag map01 will be either
#     # False, or the character to map the digit 1 (one) to.  It should be
#     # either L (el) or I (eye).
#     if map01 is not None:
#         map01 = _bytes_from_decode_data(map01)
#         assert len(map01) == 1, repr(map01)
#         s = s.translate(bytes.maketrans(b'01', b'O' + map01))
#     if casefold:
#         s = s.upper()
#     # Strip off pad characters from the right.  We need to count the pad
#     # characters because this will tell us how many null bytes to remove from
#     # the end of the decoded string.
#     l = len(s)
#     s = s.rstrip(b'=')
#     padchars = l - len(s)
#     # Now decode the full quanta
#     decoded = bytearray()
#     b32rev = _b32rev
#     for i in range(0, len(s), 8):
#         quanta = s[i: i + 8]
#         acc = 0
#         try:
#             for c in quanta:
#                 acc = (acc << 5) + b32rev[c]
#         except KeyError:
#             raise binascii.Error('Non-base32 digit found') from None
#         decoded += acc.to_bytes(5, 'big')
#     # Process the last, partial quanta
#     if l % 8 or padchars not in {0, 1, 3, 4, 6}:
#         raise binascii.Error('Incorrect padding')
#     if padchars and decoded:
#         acc <<= 5 * padchars
#         last = acc.to_bytes(5, 'big')
#         leftover = (43 - 5 * padchars) // 8  # 1: 4, 3: 3, 4: 2, 6: 1
#         decoded[-5:] = last[:leftover]
#     return bytes(decoded)

# def _bytes_from_decode_data(s):
#     if isinstance(s, str):
#         try:
#             return s.encode('ascii')
#         except UnicodeEncodeError:
#             raise ValueError('string argument should contain only ASCII characters')
#     if isinstance(s, bytes_types):
#         return s
#     try:
#         return memoryview(s).tobytes()
#     except TypeError:
#         raise TypeError("argument should be a bytes-like object or ASCII "
#                         "string, not %r" % s.__class__.__name__) from None
