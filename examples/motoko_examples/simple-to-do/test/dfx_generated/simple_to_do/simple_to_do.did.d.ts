import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ToDo {
    completed: boolean;
    description: string;
}
export interface _SERVICE {
    add_todo: ActorMethod<[string], bigint>;
    clear_completed: ActorMethod<[], undefined>;
    complete_todo: ActorMethod<[bigint], undefined>;
    get_todos: ActorMethod<[], Array<ToDo>>;
    show_todos: ActorMethod<[], string>;
}
