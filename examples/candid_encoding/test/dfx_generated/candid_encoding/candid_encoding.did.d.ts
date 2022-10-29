import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'candid_decode' : ActorMethod<[Uint8Array], string>,
  'candid_encode' : ActorMethod<[string], Uint8Array>,
}
