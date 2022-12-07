import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Entry {
    desc: string;
    phone: string;
}
export interface _SERVICE {
    insert: ActorMethod<[string, Entry], undefined>;
    lookup: ActorMethod<[string], [] | [Entry]>;
}
