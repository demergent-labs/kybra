import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Stable64GrowResult = { 'ok' : bigint } |
  { 'err' : StableMemoryError };
export type StableGrowResult = { 'ok' : number } |
  { 'err' : StableMemoryError };
export type StableMemoryError = { 'OutOfBounds' : null } |
  { 'OutOfMemory' : null };
export interface _SERVICE {
  'stable64_grow' : ActorMethod<[bigint], Stable64GrowResult>,
  'stable64_read' : ActorMethod<[bigint, bigint], Uint8Array>,
  'stable64_size' : ActorMethod<[], bigint>,
  'stable64_write' : ActorMethod<[bigint, Uint8Array], boolean>,
  'stable_bytes' : ActorMethod<[], Uint8Array>,
  'stable_grow' : ActorMethod<[number], StableGrowResult>,
  'stable_read' : ActorMethod<[number, number], Uint8Array>,
  'stable_size' : ActorMethod<[], number>,
  'stable_write' : ActorMethod<[number, Uint8Array], boolean>,
}
