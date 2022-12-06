import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CallbackStrategy {
    token: Token;
    callback: [Principal, string];
}
export interface HttpRequest {
    url: string;
    method: string;
    body: Uint8Array;
    headers: Array<[string, string]>;
}
export interface HttpResponse {
    body: Uint8Array;
    headers: Array<[string, string]>;
    upgrade: [] | [boolean];
    streaming_strategy: [] | [StreamingStrategy];
    status_code: number;
}
export interface StreamingCallbackHttpResponse {
    token: [] | [Token];
    body: Uint8Array;
}
export type StreamingStrategy = { Callback: CallbackStrategy };
export interface Token {
    arbitrary_data: string;
}
export interface _SERVICE {
    http_request: ActorMethod<[HttpRequest], HttpResponse>;
    http_request_update: ActorMethod<[HttpRequest], HttpResponse>;
    http_streaming: ActorMethod<[Token], StreamingCallbackHttpResponse>;
    test: ActorMethod<[], undefined>;
}
