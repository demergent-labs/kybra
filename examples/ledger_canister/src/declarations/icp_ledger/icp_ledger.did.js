export const idlFactory = ({ IDL }) => {
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const AccountIdentifier = IDL.Text;
  const Duration = IDL.Record({ 'secs' : IDL.Nat64, 'nanos' : IDL.Nat32 });
  const ArchiveOptions = IDL.Record({
    'num_blocks_to_archive' : IDL.Nat64,
    'trigger_threshold' : IDL.Nat64,
    'max_message_size_bytes' : IDL.Opt(IDL.Nat64),
    'cycles_for_archive_creation' : IDL.Opt(IDL.Nat64),
    'node_max_memory_size_bytes' : IDL.Opt(IDL.Nat64),
    'controller_id' : IDL.Principal,
  });
  const LedgerCanisterInitPayload = IDL.Record({
    'send_whitelist' : IDL.Vec(IDL.Principal),
    'token_symbol' : IDL.Opt(IDL.Text),
    'transfer_fee' : IDL.Opt(Tokens),
    'minting_account' : AccountIdentifier,
    'transaction_window' : IDL.Opt(Duration),
    'max_message_size_bytes' : IDL.Opt(IDL.Nat64),
    'archive_options' : IDL.Opt(ArchiveOptions),
    'initial_values' : IDL.Vec(IDL.Tuple(AccountIdentifier, Tokens)),
    'token_name' : IDL.Opt(IDL.Text),
  });
  const AccountBalanceArgs = IDL.Record({ 'account' : AccountIdentifier });
  const SubAccount = IDL.Vec(IDL.Nat8);
  const BlockHeight = IDL.Nat64;
  const NotifyCanisterArgs = IDL.Record({
    'to_subaccount' : IDL.Opt(SubAccount),
    'from_subaccount' : IDL.Opt(SubAccount),
    'to_canister' : IDL.Principal,
    'max_fee' : Tokens,
    'block_height' : BlockHeight,
  });
  const Memo = IDL.Nat64;
  const TimeStamp = IDL.Record({ 'timestamp_nanos' : IDL.Nat64 });
  const SendArgs = IDL.Record({
    'to' : AccountIdentifier,
    'fee' : Tokens,
    'memo' : Memo,
    'from_subaccount' : IDL.Opt(SubAccount),
    'created_at_time' : IDL.Opt(TimeStamp),
    'amount' : Tokens,
  });
  return IDL.Service({
    'account_balance_dfx' : IDL.Func([AccountBalanceArgs], [Tokens], ['query']),
    'notify_dfx' : IDL.Func([NotifyCanisterArgs], [], []),
    'send_dfx' : IDL.Func([SendArgs], [BlockHeight], []),
  });
};
export const init = ({ IDL }) => {
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const AccountIdentifier = IDL.Text;
  const Duration = IDL.Record({ 'secs' : IDL.Nat64, 'nanos' : IDL.Nat32 });
  const ArchiveOptions = IDL.Record({
    'num_blocks_to_archive' : IDL.Nat64,
    'trigger_threshold' : IDL.Nat64,
    'max_message_size_bytes' : IDL.Opt(IDL.Nat64),
    'cycles_for_archive_creation' : IDL.Opt(IDL.Nat64),
    'node_max_memory_size_bytes' : IDL.Opt(IDL.Nat64),
    'controller_id' : IDL.Principal,
  });
  const LedgerCanisterInitPayload = IDL.Record({
    'send_whitelist' : IDL.Vec(IDL.Principal),
    'token_symbol' : IDL.Opt(IDL.Text),
    'transfer_fee' : IDL.Opt(Tokens),
    'minting_account' : AccountIdentifier,
    'transaction_window' : IDL.Opt(Duration),
    'max_message_size_bytes' : IDL.Opt(IDL.Nat64),
    'archive_options' : IDL.Opt(ArchiveOptions),
    'initial_values' : IDL.Vec(IDL.Tuple(AccountIdentifier, Tokens)),
    'token_name' : IDL.Opt(IDL.Text),
  });
  return [LedgerCanisterInitPayload];
};
