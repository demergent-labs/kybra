import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountBalanceArgs { 'account' : AccountIdentifier }
export type AccountIdentifier = string;
export interface ArchiveOptions {
  'num_blocks_to_archive' : bigint,
  'trigger_threshold' : bigint,
  'max_message_size_bytes' : [] | [bigint],
  'cycles_for_archive_creation' : [] | [bigint],
  'node_max_memory_size_bytes' : [] | [bigint],
  'controller_id' : Principal,
}
export type BlockHeight = bigint;
export interface Duration { 'secs' : bigint, 'nanos' : number }
export interface LedgerCanisterInitPayload {
  'send_whitelist' : Array<Principal>,
  'token_symbol' : [] | [string],
  'transfer_fee' : [] | [Tokens],
  'minting_account' : AccountIdentifier,
  'transaction_window' : [] | [Duration],
  'max_message_size_bytes' : [] | [bigint],
  'archive_options' : [] | [ArchiveOptions],
  'initial_values' : Array<[AccountIdentifier, Tokens]>,
  'token_name' : [] | [string],
}
export type Memo = bigint;
export interface NotifyCanisterArgs {
  'to_subaccount' : [] | [SubAccount],
  'from_subaccount' : [] | [SubAccount],
  'to_canister' : Principal,
  'max_fee' : Tokens,
  'block_height' : BlockHeight,
}
export interface SendArgs {
  'to' : AccountIdentifier,
  'fee' : Tokens,
  'memo' : Memo,
  'from_subaccount' : [] | [SubAccount],
  'created_at_time' : [] | [TimeStamp],
  'amount' : Tokens,
}
export type SubAccount = Uint8Array;
export interface TimeStamp { 'timestamp_nanos' : bigint }
export interface Tokens { 'e8s' : bigint }
export interface Transaction {
  'memo' : Memo,
  'created_at' : BlockHeight,
  'transfer' : Transfer,
}
export type Transfer = {
    'Burn' : { 'from' : AccountIdentifier, 'amount' : Tokens }
  } |
  { 'Mint' : { 'to' : AccountIdentifier, 'amount' : Tokens } } |
  {
    'Send' : {
      'to' : AccountIdentifier,
      'from' : AccountIdentifier,
      'amount' : Tokens,
    }
  };
export interface _SERVICE {
  'account_balance_dfx' : ActorMethod<[AccountBalanceArgs], Tokens>,
  'notify_dfx' : ActorMethod<[NotifyCanisterArgs], undefined>,
  'send_dfx' : ActorMethod<[SendArgs], BlockHeight>,
}
