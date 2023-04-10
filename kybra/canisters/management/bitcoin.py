from kybra import Alias, blob, nat32, nat64, null, Opt, Record, Variant, Vec

BitcoinAddress = Alias[str]
BlockHash = Alias[blob]


class BitcoinNetwork(Variant, total=False):
    Mainnet: null
    Regtest: null
    Testnet: null


class GetBalanceArgs(Record):
    address: BitcoinAddress
    min_confirmations: Opt[nat32]
    network: BitcoinNetwork


class GetCurrentFeePercentilesArgs(Record):
    network: BitcoinNetwork


Page = Alias[blob]


class UtxosFilter(Variant, total=False):
    MinConfirmations: nat32
    Page: Page


class GetUtxosArgs(Record):
    address: BitcoinAddress
    filter: Opt[UtxosFilter]
    network: BitcoinNetwork


class Outpoint(Record):
    txid: blob
    vout: nat32


Satoshi = Alias[nat64]


class Utxo(Record):
    height: nat32
    outpoint: Outpoint
    value: Satoshi


class GetUtxosResult(Record):
    next_page: Opt[Page]
    tip_block_hash: BlockHash
    tip_height: nat32
    utxos: Vec[Utxo]


MillisatoshiPerByte = Alias[nat64]


class SendTransactionArgs(Record):
    transaction: blob
    network: BitcoinNetwork


class SendTransactionError(Variant, total=False):
    MalformedTransaction: null
    QueueFull: null
