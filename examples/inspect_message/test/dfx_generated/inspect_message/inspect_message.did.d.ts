import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'accessible' : ActorMethod<[], boolean>,
  'alsoInaccessible' : ActorMethod<[], boolean>,
  'inaccessible' : ActorMethod<[], boolean>,
}
