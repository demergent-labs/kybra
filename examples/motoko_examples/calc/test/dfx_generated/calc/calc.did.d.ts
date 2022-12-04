import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
    add: ActorMethod<[bigint], bigint>;
    clearall: ActorMethod<[], undefined>;
    div: ActorMethod<[bigint], [] | [bigint]>;
    mul: ActorMethod<[bigint], bigint>;
    sub: ActorMethod<[bigint], bigint>;
}
