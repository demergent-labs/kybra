import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Box = [] | [[string, Box]];
export interface Superhero {
    superpowers: [] | [[string, Box]];
    name: string;
}
export interface _SERVICE {
    create: ActorMethod<[Superhero], number>;
    delete_hero: ActorMethod<[number], boolean>;
    read: ActorMethod<[number], [] | [Superhero]>;
    update_: ActorMethod<[number, Superhero], boolean>;
}
