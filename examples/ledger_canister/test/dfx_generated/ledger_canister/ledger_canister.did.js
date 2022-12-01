export const idlFactory = ({ IDL }) => {
  const TransferError_TxTooOld = IDL.Record({
    'allowed_window_nanos' : IDL.Nat64,
  });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const TransferError_BadFee = IDL.Record({ 'expected_fee' : Tokens });
  const TransferError_TxDuplicate = IDL.Record({ 'duplicate_of' : IDL.Nat64 });
  const TransferError_InsufficientFunds = IDL.Record({ 'balance' : Tokens });
  const TransferError = IDL.Variant({
    'TxTooOld' : TransferError_TxTooOld,
    'BadFee' : TransferError_BadFee,
    'TxDuplicate' : TransferError_TxDuplicate,
    'TxCreatedInFuture' : IDL.Null,
    'InsufficientFunds' : TransferError_InsufficientFunds,
  });
  const TransferResult = IDL.Variant({
    'Ok' : IDL.Nat64,
    'Err' : TransferError,
  });
  const ExecuteTransferResult = IDL.Variant({
    'ok' : TransferResult,
    'err' : IDL.Text,
  });
  const GetAccountBalanceResult = IDL.Variant({
    'ok' : Tokens,
    'err' : IDL.Text,
  });
  const Archive = IDL.Record({ 'canister_id' : IDL.Principal });
  const Archives = IDL.Record({ 'archives' : IDL.Vec(Archive) });
  const GetArchivesResult = IDL.Variant({ 'ok' : Archives, 'err' : IDL.Text });
  const GetBlocksArgs = IDL.Record({
    'start' : IDL.Nat64,
    'length' : IDL.Nat64,
  });
  const Operation_Burn = IDL.Record({
    'from' : IDL.Vec(IDL.Nat8),
    'amount' : Tokens,
  });
  const Operation_Mint = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'amount' : Tokens,
  });
  const Operation_Transfer = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'fee' : Tokens,
    'from' : IDL.Vec(IDL.Nat8),
    'amount' : Tokens,
  });
  const Operation = IDL.Variant({
    'Burn' : Operation_Burn,
    'Mint' : Operation_Mint,
    'Transfer' : Operation_Transfer,
  });
  const TimeStamp = IDL.Record({ 'timestamp_nanos' : IDL.Nat64 });
  const Transaction = IDL.Record({
    'memo' : IDL.Nat64,
    'operation' : IDL.Opt(Operation),
    'created_at_time' : TimeStamp,
  });
  const Block = IDL.Record({
    'transaction' : Transaction,
    'timestamp' : TimeStamp,
    'parent_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const BlockRange = IDL.Record({ 'blocks' : IDL.Vec(Block) });
  const QueryArchiveError_BadFirstBlockIndex = IDL.Record({
    'requested_index' : IDL.Nat64,
    'first_valid_index' : IDL.Nat64,
  });
  const QueryArchiveError_Other = IDL.Record({
    'error_message' : IDL.Text,
    'error_code' : IDL.Nat64,
  });
  const QueryArchiveError = IDL.Variant({
    'BadFirstBlockIndex' : QueryArchiveError_BadFirstBlockIndex,
    'Other' : QueryArchiveError_Other,
  });
  const QueryBlocksResponse_archived_blocks = IDL.Record({
    'callback' : IDL.Func(
        [GetBlocksArgs],
        [IDL.Variant({ 'Ok' : BlockRange, 'Err' : QueryArchiveError })],
        ['query'],
      ),
    'start' : IDL.Nat64,
    'length' : IDL.Nat64,
  });
  const QueryBlocksResponse = IDL.Record({
    'certificate' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'blocks' : IDL.Vec(Block),
    'chain_length' : IDL.Nat64,
    'first_block_index' : IDL.Nat64,
    'archived_blocks' : IDL.Vec(QueryBlocksResponse_archived_blocks),
  });
  const GetBlocksResult = IDL.Variant({
    'ok' : QueryBlocksResponse,
    'err' : IDL.Text,
  });
  const GetDecimalsResult = IDL.Variant({ 'ok' : IDL.Nat32, 'err' : IDL.Text });
  const GetNameResult = IDL.Variant({ 'ok' : IDL.Text, 'err' : IDL.Text });
  const TransferFee = IDL.Record({ 'transfer_fee' : Tokens });
  const GetTransferFeeResult = IDL.Variant({
    'ok' : TransferFee,
    'err' : IDL.Text,
  });
  return IDL.Service({
    'execute_transfer' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Nat64, IDL.Opt(IDL.Nat64)],
        [ExecuteTransferResult],
        [],
      ),
    'get_account_balance' : IDL.Func([IDL.Text], [GetAccountBalanceResult], []),
    'get_address_from_principal' : IDL.Func(
        [IDL.Principal],
        [IDL.Text],
        ['query'],
      ),
    'get_archives' : IDL.Func([], [GetArchivesResult], []),
    'get_blocks' : IDL.Func([GetBlocksArgs], [GetBlocksResult], []),
    'get_decimals' : IDL.Func([], [GetDecimalsResult], []),
    'get_name' : IDL.Func([], [GetNameResult], []),
    'get_symbol' : IDL.Func([], [GetNameResult], []),
    'get_transfer_fee' : IDL.Func([], [GetTransferFeeResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
