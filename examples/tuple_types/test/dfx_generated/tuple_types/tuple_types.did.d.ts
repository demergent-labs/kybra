import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface HttpResponse { 'headers' : Array<[string, string]> }
export interface User {
  'id' : string,
  'primitive_two_tuple' : [string, bigint],
}
export interface _SERVICE {
  'complex_one_tuple_inline_param' : ActorMethod<
    [[[string, bigint]]],
    [[string, bigint]],
  >,
  'complex_one_tuple_inline_return_type' : ActorMethod<[], [[string, bigint]]>,
  'complex_one_tuple_param' : ActorMethod<
    [[[string, bigint]]],
    [[string, bigint]],
  >,
  'complex_one_tuple_return_type' : ActorMethod<[], [[string, bigint]]>,
  'complex_two_tuple_inline_param' : ActorMethod<
    [[[string, bigint], User]],
    [[string, bigint], User],
  >,
  'complex_two_tuple_inline_return_type' : ActorMethod<
    [],
    [[string, bigint], User],
  >,
  'complex_two_tuple_param' : ActorMethod<
    [[[string, bigint], User]],
    [[string, bigint], User],
  >,
  'complex_two_tuple_return_type' : ActorMethod<[], [[string, bigint], User]>,
  'primitive_one_tuple_inline_param' : ActorMethod<[[string]], [string]>,
  'primitive_one_tuple_inline_return_type' : ActorMethod<[], [string]>,
  'primitive_one_tuple_param' : ActorMethod<[[string]], [string]>,
  'primitive_one_tuple_return_type' : ActorMethod<[], [string]>,
  'primitive_three_tuple_inline_param' : ActorMethod<
    [[string, bigint, Principal]],
    [string, bigint, Principal],
  >,
  'primitive_three_tuple_inline_return_type' : ActorMethod<
    [],
    [string, bigint, Principal],
  >,
  'primitive_three_tuple_param' : ActorMethod<
    [[string, bigint, Principal]],
    [string, bigint, Principal],
  >,
  'primitive_three_tuple_return_type' : ActorMethod<
    [],
    [string, bigint, Principal],
  >,
  'primitive_two_tuple_inline_param' : ActorMethod<
    [[string, string]],
    [string, string],
  >,
  'primitive_two_tuple_inline_return_type' : ActorMethod<[], [string, string]>,
  'primitive_two_tuple_param' : ActorMethod<
    [[string, bigint]],
    [string, bigint],
  >,
  'primitive_two_tuple_return_type' : ActorMethod<[], [string, bigint]>,
  'tuple_array_params_and_return_type' : ActorMethod<
    [Array<[string, string]>],
    Array<[string, string]>,
  >,
  'tuple_array_record_field' : ActorMethod<[], HttpResponse>,
}
