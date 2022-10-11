import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'getOne' : ActorMethod<[], string>,
  'getThree' : ActorMethod<[], string>,
  'getTwo' : ActorMethod<[], string>,
  'get_external_module_message' : ActorMethod<[], bigint>,
  'get_math_message' : ActorMethod<[], bigint>,
  'sha224Hash' : ActorMethod<[string], string>,
}
