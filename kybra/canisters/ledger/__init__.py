from kybra import blob, Canister, Func, method, nat32, nat64, opt, Principal, Query, Record, Variant
from typing import TypeAlias

# Amount of tokens, measured in 10^-8 of a token.
class Tokens(Record):
    e8s: nat64

# Number of nanoseconds from the UNIX epoch in UTC timezone.
class TimeStamp(Record):
    timestamp_nanos: nat64

# TODO aliases do not work yet
# AccountIdentifier is a 32-byte array.
# The first 4 bytes is big-endian encoding of a CRC32 checksum of the last 28 bytes.
# AccountIdentifier = blob

# TODO aliases do not work yet
# Subaccount is an arbitrary 32-byte byte array.
# Ledger uses subaccounts to compute the source address, which enables one
# principal to control multiple ledger accounts.
# SubAccount = blob

# TODO aliases do not work yet
# Sequence number of a block produced by the ledger.
# BlockIndex = nat64

# TODO aliases do not work yet
# An arbitrary number associated with a transaction.
# The caller can set it in a `transfer` call as a correlation identifier.
# Memo = nat64

# Arguments for the `transfer` call.
class TransferArgs(Record):
    # Transaction memo.
    # See comments for the `Memo` type.
    memo: nat64
    # The amount that the caller wants to transfer to the destination address.
    amount: Tokens
    # The amount that the caller pays for the transaction.
    # Must be 10000 e8s.
    fee: Tokens
    # The subaccount from which the caller wants to transfer funds.
    # If null, the ledger uses the default (all zeros) subaccount to compute the source address.
    # See comments for the `SubAccount` type.
    from_subaccount: opt[blob]
    # The destination account.
    # If the transfer is successful, the balance of this address increases by `amount`.
    to: blob
    # The point in time when the caller created this request.
    # If null, the ledger uses current IC time as the timestamp.
    created_at_time: opt[TimeStamp]

class TransferError_BadFee(Record):
    expected_fee: Tokens

class TransferError_InsufficientFunds(Record):
    balance: Tokens

class TransferError_TxTooOld(Record):
    allowed_window_nanos: nat64

class TransferError_TxDuplicate(Record):
    duplicate_of: nat64

class TransferError(Variant):
    # The fee that the caller specified in the transfer request was not the one that ledger expects.
    # The caller can change the transfer fee to the `expected_fee` and retry the request.
    BadFee: TransferError_BadFee
    # The account specified by the caller doesn't have enough funds.
    InsufficentFunds: TransferError_InsufficientFunds
    # The request is too old.
    # The ledger only accepts requests created within 24 hours window.
    # This is a non-recoverable error.
    TxTooOld: TransferError_TxTooOld
    # The caller specified `created_at_time` that is too far in future.
    # The caller can retry the request later.
    TxCreatedInFuture: None
    # The ledger has already executed the request.
    # `duplicate_of` field is equal to the index of the block containing the original transaction.
    TxDuplicate: TransferError_TxDuplicate

class TransferResult(Variant):
    Ok: nat64
    Err: TransferError

# Arguments for the `account_balance` call.
class AccountBalanceArgs(Record):
    account: blob

class TransferFeeArg(Record): ...

class TransferFee(Record):
    # The fee to pay to perform a transfer
    transfer_fee: Tokens

class GetBlocksArgs(Record):
    # The index of the first block to fetch.
    start: nat64
    # Max number of blocks to fetch.
    length: nat64

class Operation_Mint(Record):
    to: blob
    amount: Tokens

# TODO Azle/Kybra CDK framework need to handle language keywords
class Operation_Burn(Record):
    from_: blob
    amount: Tokens

class Operation_Transfer(Record):
    from_: blob
    to: blob
    amount: Tokens
    fee: Tokens

class Operation(Variant):
    Mint: Operation_Mint
    Burn: Operation_Burn
    Transfer: Operation_Transfer

class Transaction(Record):
    memo: blob
    operation: opt[Operation]
    created_at_time: TimeStamp

class Block(Record):
    parent_hash: opt[blob]
    transaction: Transaction
    timestamp: TimeStamp

# A prefix of the block range specified in the [GetBlocksArgs] request.
class BlockRange(Record):
    # A prefix of the requested block range.
    # The index of the first block is equal to [GetBlocksArgs.from].
    #
    # Note that the number of blocks might be less than the requested
    # [GetBlocksArgs.len] for various reasons, for example:
    #
    # 1. The query might have hit the replica with an outdated state
    #    that doesn't have the full block range yet.
    # 2. The requested range is too large to fit into a single reply.
    #
    # NOTE: the list of blocks can be empty if:
    # 1. [GetBlocksArgs.len] was zero.
    # 2. [GetBlocksArgs.from] was larger than the last block known to the canister.
    blocks: list[Block]

class QueryArchiveError_BadFirstBlockIndex(Record):
    requested_index: nat64
    first_valid_index: nat64

class QueryArchiveError_Other(Record):
    error_code: nat64
    error_message: str

# An error indicating that the arguments passed to [QueryArchiveFn] were invalid.
class QueryArchiveError(Variant):
    # [GetBlocksArgs.from] argument was smaller than the first block
    # served by the canister that received the request.
    BadFirstBlockIndex: QueryArchiveError_BadFirstBlockIndex
    # Reserved for future use.
    Other: QueryArchiveError_Other

class QueryArchiveResult(Variant):
    # Successfully fetched zero or more blocks.
    Ok: BlockRange
    # The [GetBlocksArgs] request was invalid.
    Err: QueryArchiveError

# A function that is used for fetching archived ledger blocks.
QueryArchiveFn: TypeAlias = Func(Query[[GetBlocksArgs], QueryArchiveResult])

class QueryBlocksResponse_archived_blocks(Record):
    # The index of the first archived block that can be fetched using the callback.
    start: nat64

    # The number of blocks that can be fetch using the callback.
    length: nat64

    # The function that should be called to fetch the archived blocks.
    # The range of the blocks accessible using this function is given by [from]
    # and [len] fields above.
    callback: QueryArchiveFn

# The result of a "query_blocks" call.
#
# The structure of the result is somewhat complicated because the main ledger canister might
# not have all the blocks that the caller requested: One or more "archive" canisters might
# store some of the requested blocks.
#
# Note: as of Q4 2021 when this interface is authored, the IC doesn't support making nested
# query calls within a query call.
class QueryBlocksResponse(Record):
    # The total number of blocks in the chain.
    # If the chain length is positive, the index of the last block is `chain_len - 1`.
    chain_length: opt[blob]

    # System certificate for the hash of the latest block in the chain.
    # Only present if `query_blocks` is called in a non-replicated query context.
    certificate: opt[blob]

    # List of blocks that were available in the ledger when it processed the call.
    #
    # The blocks form a contiguous range, with the first block having index
    # [first_block_index] (see below), and the last block having index
    # [first_block_index] + len(blocks) - 1.
    #
    # The block range can be an arbitrary sub-range of the originally requested range.
    blocks: list[Block]

    # The index of the first block in "blocks".
    # If the blocks vector is empty, the exact value of this field is not specified.
    first_block_index: nat64

    # Encoding of instructions for fetching archived blocks whose indices fall into the
    # requested range.
    #
    # For each entry `e` in [archived_blocks], `[e.from, e.from + len)` is a sub-range
    # of the originally requested block range.
    archived_blocks: list[QueryBlocksResponse_archived_blocks]

class Archive(Record):
    canister_id: Principal

class Archives(Record):
    archives: list[Archive]

class SymbolResult(Record):
    symbol: str

class NameResult(Record):
    name: str

class DecimalsResult(Record):
    decimals: nat32

# TODO aliases do not work yet
# Address = str

class Ledger(Canister):
    # Transfers tokens from a subaccount of the caller to the destination address.
    # The source address is computed from the principal of the caller and the specified subaccount.
    # When successful, returns the index of the block containing the transaction.
    @method
    def transfer(self, transfer_args: TransferArgs) -> TransferResult: ...

    # Returns the amount of Tokens on the specified account.
    @method
    def account_balance(self, account_balance_args: AccountBalanceArgs) -> Tokens: ...

    # Returns the current transfer_fee.
    @method
    def transfer_fee(self, transfer_fee_arg: TransferFeeArg) -> TransferFee: ...

    # Queries blocks in the specified range.
    @method
    def query_blocks(self, get_blocks_args: GetBlocksArgs) -> QueryBlocksResponse: ...

    # Returns token symbol.
    @method
    def symbol(self) -> SymbolResult: ...

    # Returns token name.
    @method
    def name(self) -> NameResult: ...

    # Returns token decimals.
    @method
    def decimals(self) -> DecimalsResult: ...

    # Returns the existing archive canisters information.
    @method
    def archives(self) -> Archives: ...
