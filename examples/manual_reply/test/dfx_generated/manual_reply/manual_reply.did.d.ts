import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Gas = { Elemental: null } | { Mixed: null } | { Toxic: null };
export interface ManualReply {
    id: string;
    orbitals: Array<Orbital>;
    state: State;
}
export interface ManualReply_1 {
    int: bigint;
    blob: Uint8Array;
    bool: boolean;
    text: string;
    variant: Options;
}
export type Options = { Large: null } | { Small: null } | { Medium: null };
export interface Orbital {
    electrons: number;
    layer: number;
}
export interface Solid {
    element: string;
}
export type State = { Gas: Gas } | { Solid: Solid } | { Liquid: null };
export interface _SERVICE {
    manual_query: ActorMethod<[string], string>;
    manual_update: ActorMethod<[string], string>;
    query_blob: ActorMethod<[], Uint8Array>;
    query_float32: ActorMethod<[], number>;
    query_int8: ActorMethod<[], number>;
    query_nat: ActorMethod<[], bigint>;
    query_null: ActorMethod<[], null>;
    query_record: ActorMethod<[], ManualReply>;
    query_reserved: ActorMethod<[], any>;
    query_string: ActorMethod<[], string>;
    query_variant: ActorMethod<[], Gas>;
    reply_raw: ActorMethod<[], ManualReply_1>;
    update_blob: ActorMethod<[], Uint8Array>;
    update_float32: ActorMethod<[], number>;
    update_int8: ActorMethod<[], number>;
    update_nat: ActorMethod<[], bigint>;
    update_null: ActorMethod<[], null>;
    update_record: ActorMethod<[], ManualReply>;
    update_reserved: ActorMethod<[], any>;
    update_string: ActorMethod<[], string>;
    update_variant: ActorMethod<[], Gas>;
}
