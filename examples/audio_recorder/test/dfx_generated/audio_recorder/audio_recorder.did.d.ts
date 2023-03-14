import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type CreateRecordingErr = { 'InsertError' : InsertError } |
  { 'UserDoesNotExist' : Principal };
export type CreateRecordingResult = { 'ok' : Recording } |
  { 'err' : CreateRecordingErr };
export type CreateUserResult = { 'ok' : User } |
  { 'err' : InsertError };
export type DeleteRecordingError = { 'RecordingDoesNotExist' : Principal } |
  { 'InsertError' : InsertError } |
  { 'UserDoesNotExist' : Principal };
export type DeleteRecordingResult = { 'ok' : Recording } |
  { 'err' : DeleteRecordingError };
export type DeleteUserErr = { 'UserDoesNotExist' : Principal };
export type DeleteUserResult = { 'ok' : User } |
  { 'err' : DeleteUserErr };
export type InsertError = { 'ValueTooLarge' : KeyTooLarge } |
  { 'KeyTooLarge' : KeyTooLarge };
export interface KeyTooLarge { 'max' : number, 'given' : number }
export interface Recording {
  'id' : Principal,
  'audio' : Uint8Array,
  'name' : string,
  'created_at' : bigint,
  'user_id' : Principal,
}
export interface User {
  'id' : Principal,
  'recording_ids' : Array<Principal>,
  'username' : string,
  'created_at' : bigint,
}
export interface _SERVICE {
  'create_recording' : ActorMethod<
    [Uint8Array, string, Principal],
    CreateRecordingResult
  >,
  'create_user' : ActorMethod<[string], CreateUserResult>,
  'delete_recording' : ActorMethod<[Principal], DeleteRecordingResult>,
  'delete_user' : ActorMethod<[Principal], DeleteUserResult>,
  'read_recording_by_id' : ActorMethod<[Principal], [] | [Recording]>,
  'read_recordings' : ActorMethod<[], Array<Recording>>,
  'read_user_by_id' : ActorMethod<[Principal], [] | [User]>,
  'read_users' : ActorMethod<[], Array<User>>,
}
