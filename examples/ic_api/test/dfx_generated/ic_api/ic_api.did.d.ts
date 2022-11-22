import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'caller' : ActorMethod<[], Principal>,
  'performance_counter' : ActorMethod<[], bigint>,
}
