import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type IntQueryResult = { ok: bigint } | { err: string };
export type StringQueryResult = { ok: string } | { err: string };
export interface _SERVICE {
    deep_query: ActorMethod<[], StringQueryResult>;
    inc_canister1: ActorMethod<[], IntQueryResult>;
    inc_canister2: ActorMethod<[], IntQueryResult>;
    inc_counter: ActorMethod<[], bigint>;
    manual_query: ActorMethod<[], StringQueryResult>;
    simple_composite_query: ActorMethod<[], StringQueryResult>;
    simple_query: ActorMethod<[], StringQueryResult>;
    simple_update: ActorMethod<[], StringQueryResult>;
    update_query: ActorMethod<[], StringQueryResult>;
}
