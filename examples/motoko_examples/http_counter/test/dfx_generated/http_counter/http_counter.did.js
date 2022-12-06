export const idlFactory = ({ IDL }) => {
    const HttpRequest = IDL.Record({
        url: IDL.Text,
        method: IDL.Text,
        body: IDL.Vec(IDL.Nat8),
        headers: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))
    });
    const Token = IDL.Record({ arbitrary_data: IDL.Text });
    const CallbackStrategy = IDL.Record({
        token: Token,
        callback: IDL.Func(
            [Token],
            [IDL.Record({ token: IDL.Opt(Token), body: IDL.Vec(IDL.Nat8) })],
            ['query']
        )
    });
    const StreamingStrategy = IDL.Variant({ Callback: CallbackStrategy });
    const HttpResponse = IDL.Record({
        body: IDL.Vec(IDL.Nat8),
        headers: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
        upgrade: IDL.Opt(IDL.Bool),
        streaming_strategy: IDL.Opt(StreamingStrategy),
        status_code: IDL.Nat16
    });
    const StreamingCallbackHttpResponse = IDL.Record({
        token: IDL.Opt(Token),
        body: IDL.Vec(IDL.Nat8)
    });
    return IDL.Service({
        http_request: IDL.Func([HttpRequest], [HttpResponse], ['query']),
        http_request_update: IDL.Func([HttpRequest], [HttpResponse], []),
        http_streaming: IDL.Func(
            [Token],
            [StreamingCallbackHttpResponse],
            ['query']
        ),
        test: IDL.Func([], [], ['query'])
    });
};
export const init = ({ IDL }) => {
    return [];
};
