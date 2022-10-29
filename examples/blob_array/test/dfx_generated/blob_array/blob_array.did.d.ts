import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'get_blob' : ActorMethod<[], Uint8Array>,
  'get_blobs' : ActorMethod<[], Array<Uint8Array>>,
}
