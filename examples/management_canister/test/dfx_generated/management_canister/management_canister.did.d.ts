import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type CanisterStatus = { 'stopped' : null } |
  { 'stopping' : null } |
  { 'running' : null };
export interface CanisterStatusArgs { 'canister_id' : Principal }
export interface CanisterStatusResult {
  'status' : CanisterStatus,
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : DefiniteCanisterSettings,
  'module_hash' : [] | [Uint8Array],
}
export interface CreateCanisterResult { 'canister_id' : Principal }
export type DefaultResult = { 'ok' : boolean } |
  { 'err' : string };
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export type ExecuteCreateCanisterResult = { 'ok' : CanisterStatusArgs } |
  { 'err' : string };
export type GetCanisterStatusResult = { 'ok' : CanisterStatusResult } |
  { 'err' : string };
export type RawRandResult = { 'ok' : Uint8Array } |
  { 'err' : string };
export interface _SERVICE {
  'execute_create_canister' : ActorMethod<[], ExecuteCreateCanisterResult>,
  'execute_delete_canister' : ActorMethod<[Principal], DefaultResult>,
  'execute_install_code' : ActorMethod<[Principal, Uint8Array], DefaultResult>,
  'execute_start_canister' : ActorMethod<[Principal], DefaultResult>,
  'execute_stop_canister' : ActorMethod<[Principal], DefaultResult>,
  'execute_uninstall_code' : ActorMethod<[Principal], DefaultResult>,
  'execute_update_settings' : ActorMethod<[Principal], DefaultResult>,
  'get_canister_status' : ActorMethod<
    [CanisterStatusArgs],
    GetCanisterStatusResult,
  >,
  'get_created_canister_id' : ActorMethod<[], Principal>,
  'get_raw_rand' : ActorMethod<[], RawRandResult>,
  'provisional_create_canister_with_cycles' : ActorMethod<
    [],
    ExecuteCreateCanisterResult,
  >,
  'provisional_top_up_canister' : ActorMethod<
    [Principal, bigint],
    DefaultResult,
  >,
}
