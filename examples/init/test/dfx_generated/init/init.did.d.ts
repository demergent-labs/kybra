import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Reaction = { 'Fire' : null } |
  { 'Wave' : null };
export interface User { 'id' : string }
export interface _SERVICE {
  'getOwner' : ActorMethod<[], [] | [Principal]>,
  'getReaction' : ActorMethod<[], [] | [Reaction]>,
  'getUser' : ActorMethod<[], [] | [User]>,
}
