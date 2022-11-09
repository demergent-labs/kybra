from kybra import Async, CanisterResult, nat32, nat64, opt, Principal, query, update, Variant
from kybra.canisters.ledger import Archives, DecimalsResult, GetBlocksArgs, Ledger, NameResult, QueryBlocksResponse, SymbolResult, Tokens, TransferFee, TransferResult

icp_canister = Ledger(Principal.from_str('ryjl3-tyaaa-aaaaa-aaaba-cai'))

class ExecuteTransferResult(Variant, total=False):
    ok: TransferResult
    err: str

@update
def execute_transfer(
    to: str,
    amount: nat64,
    fee: nat64,
    created_at_time: opt[nat64]
) -> Async[ExecuteTransferResult]:
    transfer_result_canister_result: CanisterResult[TransferResult] = yield icp_canister.transfer({
        'memo': 0,
        'amount': {
            'e8s': amount
        },
        'fee': {
            'e8s': fee
        },
        'from_subaccount': None,
        'to': to.encode('utf-8'), # TODO not sure the best way to do this
        'created_at_time': None if created_at_time is None else {
            'timestamp_nanos': created_at_time
        }
    })

    if transfer_result_canister_result.err is not None:
        return {
            'err': transfer_result_canister_result.err
        }

    transfer_result = transfer_result_canister_result.ok

    return {
        'ok': transfer_result
    }

class GetAccountBalanceResult(Variant, total=False):
    ok: Tokens
    err: str

@update
def get_account_balance(address: str) -> Async[GetAccountBalanceResult]:
    tokens_canister_result: CanisterResult[Tokens] = yield icp_canister.account_balance({
        'account': address.encode('utf-8')
    })

    if tokens_canister_result.err is not None:
        return {
            'err': tokens_canister_result.err
        }

    tokens = tokens_canister_result.ok

    return {
        'ok': tokens
    }

class GetTransferFeeResult(Variant, total=False):
    ok: TransferFee
    err: str

@update
def get_transfer_fee() -> Async[GetTransferFeeResult]:
    transfer_fee_canister_result: CanisterResult[TransferFee] = yield icp_canister.transfer_fee({});

    if transfer_fee_canister_result.err is not None:
        return {
            'err': transfer_fee_canister_result.err
        }

    transfer_fee = transfer_fee_canister_result.ok

    return {
        'ok': transfer_fee
    }

class GetBlocksResult(Variant, total=False):
    ok: QueryBlocksResponse
    err: str

@update
def get_blocks(get_blocks_args: GetBlocksArgs) -> Async[GetBlocksResult]:
    canister_result: CanisterResult[QueryBlocksResponse] = yield icp_canister.query_blocks(get_blocks_args)

    if canister_result.err is not None:
        return {
            'err': canister_result.err
        }

    get_blocks_result = canister_result.ok

    return {
        'ok': get_blocks_result
    }

class GetSymbolResult(Variant, total=False):
    ok: str
    err: str

@update
def get_symbol() -> Async[GetSymbolResult]:
    symbol_result_canister_result: CanisterResult[SymbolResult] = yield icp_canister.symbol();

    if symbol_result_canister_result.err is not None:
        return {
            'err': symbol_result_canister_result.err
        }

    symbol_result = symbol_result_canister_result.ok

    return {
        'ok': symbol_result['symbol']
    }

class GetNameResult(Variant, total=False):
    ok: str
    err: str

@update
def get_name() -> Async[GetNameResult]:
    name_result_canister_result: CanisterResult[NameResult] = yield icp_canister.name()

    if name_result_canister_result.err is not None:
        return {
            'err': name_result_canister_result.err
        }

    name_result = name_result_canister_result.ok

    return {
        'ok': name_result['name']
    }

class GetDecimalsResult(Variant, total=False):
    ok: nat32
    err: str

@update
def get_decimals() -> Async[GetDecimalsResult]:
    decimals_result_canister_result: CanisterResult[DecimalsResult] = yield icp_canister.decimals()

    if decimals_result_canister_result.err is not None:
        return {
            'err': decimals_result_canister_result.err
        }

    decimals_result = decimals_result_canister_result.ok

    return {
        'ok': decimals_result['decimals']
    }

class GetArchivesResult(Variant, total=False):
    ok: Archives
    err: str

@update
def get_archives() -> Async[GetArchivesResult]:
    archives_canister_result: CanisterResult[Archives] = yield icp_canister.archives()

    if archives_canister_result.err is not None:
        return {
            'err': archives_canister_result.err
        }

    archives = archives_canister_result.ok

    return {
        'ok': archives
    }

@query
def get_address_from_principal(principal: Principal) -> str:
    return principal.to_account_id().to_str()
