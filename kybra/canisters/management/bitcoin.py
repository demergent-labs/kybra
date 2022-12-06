from kybra import blob, nat32, nat64, opt, Record, Variant

BitcoinAddress = str
BlockHash = blob


class BitcoinNetwork(Variant, total=False):
    Mainnet: None
    Regtest: None
    Testnet: None


class GetBalanceArgs(Record):
    address: BitcoinAddress
    min_confirmations: opt[nat32]
    network: BitcoinNetwork


class GetCurrentFeePercentilesArgs(Record):
    network: BitcoinNetwork


Page = blob


class UtxosFilter(Variant, total=False):
    MinConfirmations: nat32
    Page: Page


class GetUtxosArgs(Record):
    address: BitcoinAddress
    filter: opt[UtxosFilter]
    network: BitcoinNetwork


class Outpoint(Record):
    txid: blob
    vout: nat32


Satoshi = nat64


class Utxo(Record):
    height: nat32
    outpoint: Outpoint
    value: Satoshi


class GetUtxosResult(Record):
    next_page: opt[Page]
    tip_block_hash: BlockHash
    tip_height: nat32
    utxos: list[Utxo]


MillisatoshiPerByte = nat64


class SendTransactionArgs(Record):
    transaction: blob
    network: BitcoinNetwork


class SendTransactionError(Variant, total=False):
    MalformedTransaction: None
    QueueFull: None
