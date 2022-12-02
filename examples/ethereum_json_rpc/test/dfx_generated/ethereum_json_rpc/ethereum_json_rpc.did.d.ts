import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface HttpHeader {
    value: string;
    name: string;
}
export interface HttpResponse {
    status: bigint;
    body: Uint8Array;
    headers: Array<HttpHeader>;
}
export interface HttpTransformArgs {
    context: Uint8Array;
    response: HttpResponse;
}
export interface _SERVICE {
    eth_get_balance: ActorMethod<[string], string>;
    eth_get_block_by_number: ActorMethod<[number], string>;
    eth_transform: ActorMethod<[HttpTransformArgs], HttpResponse>;
}
