import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface PartiallyNullRecord {
  'first_item' : bigint,
  'third_item' : bigint,
  'second_item' : null,
}
export interface ThreeNullRecord {
  'first_item' : null,
  'third_item' : null,
  'second_item' : null,
}
export interface TwoNullRecord { 'first_item' : null, 'second_item' : null }
export interface _SERVICE {
  'get_large_null_record' : ActorMethod<[], ThreeNullRecord>,
  'get_partially_null_record' : ActorMethod<[], PartiallyNullRecord>,
  'get_small_null_record' : ActorMethod<[], TwoNullRecord>,
  'null_function' : ActorMethod<[null], null>,
  'set_large_null_record' : ActorMethod<[ThreeNullRecord], ThreeNullRecord>,
  'set_partially_null_record' : ActorMethod<
    [PartiallyNullRecord],
    PartiallyNullRecord,
  >,
  'set_small_null_record' : ActorMethod<[TwoNullRecord], TwoNullRecord>,
}
