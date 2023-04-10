from kybra import (
    Async,
    CallResult,
    match,
    nat32,
    nat64,
    Opt,
    Principal,
    query,
    update,
    Variant,
)
from kybra.canisters.ledger import (
    Address,
    Archives,
    DecimalsResult,
    GetBlocksArgs,
    Ledger,
    NameResult,
    QueryBlocksResponse,
    SymbolResult,
    Tokens,
    TransferFee,
    TransferResult,
)

icp_canister = Ledger(Principal.from_str("r7inp-6aaaa-aaaaa-aaabq-cai"))


class ExecuteTransferResult(Variant, total=False):
    Ok: TransferResult
    Err: str


@update
def execute_transfer(
    to: Address, amount: nat64, fee: nat64, created_at_time: Opt[nat64]
) -> Async[ExecuteTransferResult]:
    transfer_result_call_result: CallResult[
        TransferResult
    ] = yield icp_canister.transfer(
        {
            "memo": 0,
            "amount": {"e8s": amount},
            "fee": {"e8s": fee},
            "from_subaccount": None,
            "to": bytes.fromhex(to),
            "created_at_time": None
            if created_at_time is None
            else {"timestamp_nanos": created_at_time},
        }
    )

    return match(
        transfer_result_call_result,
        {
            "Ok": lambda transfer_result: {"Ok": transfer_result},
            "Err": lambda err: {"Err": err},
        },
    )


class GetAccountBalanceResult(Variant, total=False):
    Ok: Tokens
    Err: str


@update
def get_account_balance(address: Address) -> Async[GetAccountBalanceResult]:
    tokens_call_result: CallResult[Tokens] = yield icp_canister.account_balance(
        {"account": bytes.fromhex(address)}
    )

    return match(
        tokens_call_result,
        {
            "Ok": lambda tokens: {"Ok": tokens},
            "Err": lambda err: {"Err": err},
        },
    )


class GetTransferFeeResult(Variant, total=False):
    Ok: TransferFee
    Err: str


@update
def get_transfer_fee() -> Async[GetTransferFeeResult]:
    transfer_fee_call_result: CallResult[TransferFee] = yield icp_canister.transfer_fee(
        {}
    )

    return match(
        transfer_fee_call_result,
        {
            "Ok": lambda transfer_fee: {"Ok": transfer_fee},
            "Err": lambda err: {"Err": err},
        },
    )


class GetBlocksResult(Variant, total=False):
    Ok: QueryBlocksResponse
    Err: str


@update
def get_blocks(get_blocks_args: GetBlocksArgs) -> Async[GetBlocksResult]:
    call_result: CallResult[QueryBlocksResponse] = yield icp_canister.query_blocks(
        get_blocks_args
    )

    return match(
        call_result,
        {
            "Ok": lambda get_blocks_result: {"Ok": get_blocks_result},
            "Err": lambda err: {"Err": err},
        },
    )


class GetSymbolResult(Variant, total=False):
    Ok: str
    Err: str


@update
def get_symbol() -> Async[GetSymbolResult]:
    symbol_result_call_result: CallResult[SymbolResult] = yield icp_canister.symbol()

    return match(
        symbol_result_call_result,
        {
            "Ok": lambda symbol_result: {"Ok": symbol_result["symbol"]},
            "Err": lambda err: {"Err": err},
        },
    )


class GetNameResult(Variant, total=False):
    Ok: str
    Err: str


@update
def get_name() -> Async[GetNameResult]:
    name_result_call_result: CallResult[NameResult] = yield icp_canister.name()

    return match(
        name_result_call_result,
        {
            "Ok": lambda name_result: {"Ok": name_result["name"]},
            "Err": lambda err: {"Err": err},
        },
    )


class GetDecimalsResult(Variant, total=False):
    Ok: nat32
    Err: str


@update
def get_decimals() -> Async[GetDecimalsResult]:
    decimals_result_call_result: CallResult[
        DecimalsResult
    ] = yield icp_canister.decimals()

    return match(
        decimals_result_call_result,
        {
            "Ok": lambda decimals_result: {"Ok": decimals_result["decimals"]},
            "Err": lambda err: {"Err": err},
        },
    )


class GetArchivesResult(Variant, total=False):
    Ok: Archives
    Err: str


@update
def get_archives() -> Async[GetArchivesResult]:
    archives_call_result: CallResult[Archives] = yield icp_canister.archives()

    return match(
        archives_call_result,
        {
            "Ok": lambda archives: {"Ok": archives},
            "Err": lambda err: {"Err": err},
        },
    )


@query
def get_address_from_principal(principal: Principal) -> str:
    return principal.to_account_id().to_str()[2:]
