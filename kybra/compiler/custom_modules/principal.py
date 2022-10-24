
# The Principal code is taken from https://github.com/rocklabs-io/ic-py/blob/main/ic/principal.py

# MIT License

# Copyright (c) 2021 Rocklabs

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

# principal type: https://github.com/dfinity/ic-types/blob/main/src/principal.rs

import zlib
import math
# import base64 # TODO we probably need to remove this
import binascii
import hashlib
# from enum import Enum # TODO we probably need to remove this

CRC_LENGTH_IN_BYTES = 4
HASH_LENGTH_IN_BYTES = 28
MAX_LENGTH_IN_BYTES = 29

# class PrincipalClass(Enum):
#     OpaqueId = 1
#     SelfAuthenticating = 2
#     DerivedId = 3
#     Anonymous = 4
#     # Unassigned

class Principal:
    def __init__(self, bytes: bytes = b''):
        self._len = len(bytes)
        self._bytes = bytes
        self.hex = str(self._bytes.hex()).upper()
        self._isPrincipal = True

    @staticmethod
    def management_canister():
        return Principal()

    @staticmethod
    def self_authenticating(pubkey):
        if isinstance(pubkey, str):
            pubkey = bytes.fromhex(pubkey)
        hash_ = hashlib.sha224(pubkey).digest()
        hash_ += bytes([2])
        return Principal(bytes = hash_)

    @staticmethod
    def anonymous():
        return Principal(bytes = b'\x04')

    @property
    def len(self):
        return self._len

    @property
    def bytes(self):
        return self._bytes

    @property
    def isPrincipal(self):
        return self._isPrincipal

    @staticmethod
    def from_str(s: str):
        s1 = s.replace('-', '')
        pad_len = math.ceil(len(s1) / 8) * 8 - len(s1)
        b = b32decode(s1.upper().encode() + b'=' * pad_len)
        if len(b) < CRC_LENGTH_IN_BYTES:
            raise Exception("principal length error")
        p = Principal(bytes = b[CRC_LENGTH_IN_BYTES:])
        if not p.to_str() == s:
            raise Exception("principal format error")
        return p

    @staticmethod
    def from_hex(s: str):
        return Principal(bytes.fromhex(s.lower()))

    def to_str(self):
        checksum = zlib.crc32(self._bytes) & 0xFFFFFFFF
        b = b''
        b += checksum.to_bytes(CRC_LENGTH_IN_BYTES, byteorder='big')
        b += self.bytes
        s = b32encode(b).decode('utf-8').lower().replace('=', '')
        ret = ''
        while len(s) > 5:
            ret += s[:5]
            ret += '-'
            s = s[5:]
        ret += s
        return ret

    def to_account_id(self, sub_account: int = 0):
        return AccountIdentifier.new(self, sub_account)

    def __repr__(self):
        return "Principal(" + self.to_str() + ")"

    def __str__(self):
        return self.to_str()

class AccountIdentifier:
    def __init__(self, hash: bytes) -> None:
        assert len(hash) == 32
        self._hash = hash

    def to_str(self):
        return '0x' + self._hash.hex()

    def __repr__(self):
        return "Account(" + self.to_str() + ")"

    def __str__(self):
        return self.to_str()

    @property
    def bytes(self) -> bytes:
        return self._hash

    @staticmethod
    def new(principal: Principal, sub_account: int = 0):
        sha224 = hashlib.sha224()
        sha224.update(b'\x0Aaccount-id')
        sha224.update(principal.bytes)

        sub_account = sub_account.to_bytes(32, byteorder='big')
        sha224.update(sub_account)
        hash = sha224.digest()
        checksum = zlib.crc32(hash) & 0xFFFFFFFF
        account = checksum.to_bytes(CRC_LENGTH_IN_BYTES, byteorder='big') + hash
        return AccountIdentifier(account)

# Everything below taken from RustPython/Lib/base64.py

# These functions have been modified from the RustPython code, which was most likely created mostly from the CPython code
# The license to CPython can be found here: kybra/licenses/CPYTHON_LICENSE
# The license to RustPython can be found here: kybra/licenses/RUST_PYTHON_LICENSE

# Base32 encoding/decoding must be done in Python
_b32alphabet = b'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567'
_b32tab2 = None
_b32rev = None

bytes_types = (bytes, bytearray)  # Types acceptable as binary data

def b32encode(s):
    """Encode the bytes-like object s using Base32 and return a bytes object.
    """
    global _b32tab2
    # Delay the initialization of the table to not waste memory
    # if the function is never called
    if _b32tab2 is None:
        b32tab = [bytes((i,)) for i in _b32alphabet]
        _b32tab2 = [a + b for a in b32tab for b in b32tab]
        b32tab = None

    if not isinstance(s, bytes_types):
        s = memoryview(s).tobytes()
    leftover = len(s) % 5
    # Pad the last quantum with zero bits if necessary
    if leftover:
        s = s + b'\0' * (5 - leftover)  # Don't use += !
    encoded = bytearray()
    from_bytes = int.from_bytes
    b32tab2 = _b32tab2
    for i in range(0, len(s), 5):
        c = from_bytes(s[i: i + 5], 'big')
        encoded += (b32tab2[c >> 30] +           # bits 1 - 10
                    b32tab2[(c >> 20) & 0x3ff] + # bits 11 - 20
                    b32tab2[(c >> 10) & 0x3ff] + # bits 21 - 30
                    b32tab2[c & 0x3ff]           # bits 31 - 40
                   )
    # Adjust for any leftover partial quanta
    if leftover == 1:
        encoded[-6:] = b'======'
    elif leftover == 2:
        encoded[-4:] = b'===='
    elif leftover == 3:
        encoded[-3:] = b'==='
    elif leftover == 4:
        encoded[-1:] = b'='
    return bytes(encoded)

def b32decode(s, casefold=False, map01=None):
    """Decode the Base32 encoded bytes-like object or ASCII string s.

    Optional casefold is a flag specifying whether a lowercase alphabet is
    acceptable as input.  For security purposes, the default is False.

    RFC 3548 allows for optional mapping of the digit 0 (zero) to the
    letter O (oh), and for optional mapping of the digit 1 (one) to
    either the letter I (eye) or letter L (el).  The optional argument
    map01 when not None, specifies which letter the digit 1 should be
    mapped to (when map01 is not None, the digit 0 is always mapped to
    the letter O).  For security purposes the default is None, so that
    0 and 1 are not allowed in the input.

    The result is returned as a bytes object.  A binascii.Error is raised if
    the input is incorrectly padded or if there are non-alphabet
    characters present in the input.
    """
    global _b32rev
    # Delay the initialization of the table to not waste memory
    # if the function is never called
    if _b32rev is None:
        _b32rev = {v: k for k, v in enumerate(_b32alphabet)}
    s = _bytes_from_decode_data(s)
    if len(s) % 8:
        raise binascii.Error('Incorrect padding')
    # Handle section 2.4 zero and one mapping.  The flag map01 will be either
    # False, or the character to map the digit 1 (one) to.  It should be
    # either L (el) or I (eye).
    if map01 is not None:
        map01 = _bytes_from_decode_data(map01)
        assert len(map01) == 1, repr(map01)
        s = s.translate(bytes.maketrans(b'01', b'O' + map01))
    if casefold:
        s = s.upper()
    # Strip off pad characters from the right.  We need to count the pad
    # characters because this will tell us how many null bytes to remove from
    # the end of the decoded string.
    l = len(s)
    s = s.rstrip(b'=')
    padchars = l - len(s)
    # Now decode the full quanta
    decoded = bytearray()
    b32rev = _b32rev
    for i in range(0, len(s), 8):
        quanta = s[i: i + 8]
        acc = 0
        try:
            for c in quanta:
                acc = (acc << 5) + b32rev[c]
        except KeyError:
            raise binascii.Error('Non-base32 digit found') from None
        decoded += acc.to_bytes(5, 'big')
    # Process the last, partial quanta
    if l % 8 or padchars not in {0, 1, 3, 4, 6}:
        raise binascii.Error('Incorrect padding')
    if padchars and decoded:
        acc <<= 5 * padchars
        last = acc.to_bytes(5, 'big')
        leftover = (43 - 5 * padchars) // 8  # 1: 4, 3: 3, 4: 2, 6: 1
        decoded[-5:] = last[:leftover]
    return bytes(decoded)

def _bytes_from_decode_data(s):
    if isinstance(s, str):
        try:
            return s.encode('ascii')
        except UnicodeEncodeError:
            raise ValueError('string argument should contain only ASCII characters')
    if isinstance(s, bytes_types):
        return s
    try:
        return memoryview(s).tobytes()
    except TypeError:
        raise TypeError("argument should be a bytes-like object or ASCII "
                        "string, not %r" % s.__class__.__name__) from None
