import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'get_current_message' : ActorMethod<[], string>,
  'simple_update' : ActorMethod<[string], string>,
}
