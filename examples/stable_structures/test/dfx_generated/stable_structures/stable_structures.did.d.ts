import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface BlogPost { 'title' : string }
export type Reaction = { 'Sad' : null } |
  { 'Happy' : null };
export interface User { 'username' : string, 'blog_posts' : Array<BlogPost> }
export interface _SERVICE {
  'get_stable_map_0' : ActorMethod<[number], [] | [string]>,
  'get_stable_map_1' : ActorMethod<[number], [] | [Uint8Array]>,
  'get_stable_map_10' : ActorMethod<[number], [] | [[] | [boolean]]>,
  'get_stable_map_11' : ActorMethod<[bigint], [] | [User]>,
  'get_stable_map_12' : ActorMethod<[Uint8Array], [] | [Reaction]>,
  'get_stable_map_13' : ActorMethod<[string], [] | [Principal]>,
  'get_stable_map_2' : ActorMethod<[number], [] | [bigint]>,
  'get_stable_map_3' : ActorMethod<[Reaction], [] | [bigint]>,
  'get_stable_map_4' : ActorMethod<[User], [] | [number]>,
  'get_stable_map_5' : ActorMethod<[[] | [string]], [] | [number]>,
  'get_stable_map_6' : ActorMethod<[BigUint64Array], [] | [boolean]>,
  'get_stable_map_7' : ActorMethod<[null], [] | [null]>,
  'get_stable_map_8' : ActorMethod<[boolean], [] | [null]>,
  'get_stable_map_9' : ActorMethod<[number], [] | [Array<string>]>,
  'set_stable_map_0' : ActorMethod<[number, string], undefined>,
  'set_stable_map_1' : ActorMethod<[number, Uint8Array], undefined>,
  'set_stable_map_10' : ActorMethod<[number, [] | [boolean]], undefined>,
  'set_stable_map_11' : ActorMethod<[bigint, User], undefined>,
  'set_stable_map_12' : ActorMethod<[Uint8Array, Reaction], undefined>,
  'set_stable_map_13' : ActorMethod<[string, Principal], undefined>,
  'set_stable_map_2' : ActorMethod<[number, bigint], undefined>,
  'set_stable_map_3' : ActorMethod<[Reaction, bigint], undefined>,
  'set_stable_map_4' : ActorMethod<[User, number], undefined>,
  'set_stable_map_5' : ActorMethod<[[] | [string], number], undefined>,
  'set_stable_map_6' : ActorMethod<[BigUint64Array, boolean], undefined>,
  'set_stable_map_7' : ActorMethod<[null, null], undefined>,
  'set_stable_map_8' : ActorMethod<[boolean, null], undefined>,
  'set_stable_map_9' : ActorMethod<[number, Array<string>], undefined>,
}
