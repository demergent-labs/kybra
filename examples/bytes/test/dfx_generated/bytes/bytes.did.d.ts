import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'get_bytes' : ActorMethod<[Uint8Array], Uint8Array>,
}
