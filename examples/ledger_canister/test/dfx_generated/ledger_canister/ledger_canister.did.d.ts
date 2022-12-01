import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Archive { 'canister_id' : Principal }
export interface Archives { 'archives' : Array<Archive> }
export interface Block {
  'transaction' : Transaction,
  'timestamp' : TimeStamp,
  'parent_hash' : [] | [Uint8Array],
}
export interface BlockRange { 'blocks' : Array<Block> }
export type ExecuteTransferResult = { 'ok' : TransferResult } |
  { 'err' : string };
export type GetAccountBalanceResult = { 'ok' : Tokens } |
  { 'err' : string };
export type GetArchivesResult = { 'ok' : Archives } |
  { 'err' : string };
export interface GetBlocksArgs { 'start' : bigint, 'length' : bigint }
export type GetBlocksResult = { 'ok' : QueryBlocksResponse } |
  { 'err' : string };
export type GetDecimalsResult = { 'ok' : number } |
  { 'err' : string };
export type GetNameResult = { 'ok' : string } |
  { 'err' : string };
export type GetTransferFeeResult = { 'ok' : TransferFee } |
  { 'err' : string };
export type Operation = { 'Burn' : Operation_Burn } |
  { 'Mint' : Operation_Mint } |
  { 'Transfer' : Operation_Transfer };
export interface Operation_Burn { 'from' : Uint8Array, 'amount' : Tokens }
export interface Operation_Mint { 'to' : Uint8Array, 'amount' : Tokens }
export interface Operation_Transfer {
  'to' : Uint8Array,
  'fee' : Tokens,
  'from' : Uint8Array,
  'amount' : Tokens,
}
export type QueryArchiveError = {
    'BadFirstBlockIndex' : QueryArchiveError_BadFirstBlockIndex
  } |
  { 'Other' : QueryArchiveError_Other };
export interface QueryArchiveError_BadFirstBlockIndex {
  'requested_index' : bigint,
  'first_valid_index' : bigint,
}
export interface QueryArchiveError_Other {
  'error_message' : string,
  'error_code' : bigint,
}
export interface QueryBlocksResponse {
  'certificate' : [] | [Uint8Array],
  'blocks' : Array<Block>,
  'chain_length' : bigint,
  'first_block_index' : bigint,
  'archived_blocks' : Array<QueryBlocksResponse_archived_blocks>,
}
export interface QueryBlocksResponse_archived_blocks {
  'callback' : [Principal, string],
  'start' : bigint,
  'length' : bigint,
}
export interface TimeStamp { 'timestamp_nanos' : bigint }
export interface Tokens { 'e8s' : bigint }
export interface Transaction {
  'memo' : bigint,
  'operation' : [] | [Operation],
  'created_at_time' : TimeStamp,
}
export type TransferError = { 'TxTooOld' : TransferError_TxTooOld } |
  { 'BadFee' : TransferError_BadFee } |
  { 'TxDuplicate' : TransferError_TxDuplicate } |
  { 'TxCreatedInFuture' : null } |
  { 'InsufficientFunds' : TransferError_InsufficientFunds };
export interface TransferError_BadFee { 'expected_fee' : Tokens }
export interface TransferError_InsufficientFunds { 'balance' : Tokens }
export interface TransferError_TxDuplicate { 'duplicate_of' : bigint }
export interface TransferError_TxTooOld { 'allowed_window_nanos' : bigint }
export interface TransferFee { 'transfer_fee' : Tokens }
export type TransferResult = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export interface _SERVICE {
  'execute_transfer' : ActorMethod<
    [string, bigint, bigint, [] | [bigint]],
    ExecuteTransferResult,
  >,
  'get_account_balance' : ActorMethod<[string], GetAccountBalanceResult>,
  'get_address_from_principal' : ActorMethod<[Principal], string>,
  'get_archives' : ActorMethod<[], GetArchivesResult>,
  'get_blocks' : ActorMethod<[GetBlocksArgs], GetBlocksResult>,
  'get_decimals' : ActorMethod<[], GetDecimalsResult>,
  'get_name' : ActorMethod<[], GetNameResult>,
  'get_symbol' : ActorMethod<[], GetNameResult>,
  'get_transfer_fee' : ActorMethod<[], GetTransferFeeResult>,
}
