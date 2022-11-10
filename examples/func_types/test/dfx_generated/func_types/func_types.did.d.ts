import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type ComplexFunc = ActorMethod<
    [
        {
            id: string;
            basic_func: [Principal, string];
            complex_func: [Principal, string];
        },
        (
            | { Bad: null }
            | { ComplexFunc: [Principal, string] }
            | { Good: null }
            | { BasicFunc: [Principal, string] }
        )
    ],
    bigint
>;
export interface _SERVICE {
    basic_func_param: ActorMethod<[[Principal, string]], [Principal, string]>;
    basic_func_param_array: ActorMethod<
        [Array<[Principal, string]>],
        Array<[Principal, string]>
    >;
    basic_func_return_type: ActorMethod<[], [Principal, string]>;
    basic_func_return_type_array: ActorMethod<[], Array<[Principal, string]>>;
    complex_func_param: ActorMethod<[[Principal, string]], [Principal, string]>;
    complex_func_return_type: ActorMethod<[], [Principal, string]>;
}
