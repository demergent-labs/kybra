export const idlFactory = ({ IDL }) => {
    const HttpHeader = IDL.Record({ value: IDL.Text, name: IDL.Text });
    const HttpResponse = IDL.Record({
        status: IDL.Nat,
        body: IDL.Vec(IDL.Nat8),
        headers: IDL.Vec(HttpHeader)
    });
    const HttpTransformArgs = IDL.Record({
        context: IDL.Vec(IDL.Nat8),
        response: HttpResponse
    });
    return IDL.Service({
        eth_get_balance: IDL.Func([IDL.Text], [IDL.Text], []),
        eth_get_block_by_number: IDL.Func([IDL.Nat32], [IDL.Text], []),
        eth_transform: IDL.Func([HttpTransformArgs], [HttpResponse], ['query'])
    });
};
export const init = ({ IDL }) => {
    return [IDL.Text];
};
