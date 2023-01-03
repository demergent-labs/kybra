export const idlFactory = ({ IDL }) => {
    const StringQueryResult = IDL.Variant({ ok: IDL.Text, err: IDL.Text });
    const IntQueryResult = IDL.Variant({ ok: IDL.Int, err: IDL.Text });
    return IDL.Service({
        deep_query: IDL.Func([], [StringQueryResult], ['query']),
        inc_canister1: IDL.Func([], [IntQueryResult], ['query']),
        inc_canister2: IDL.Func([], [IntQueryResult], ['query']),
        inc_counter: IDL.Func([], [IDL.Int], ['query']),
        manual_query: IDL.Func([], [StringQueryResult], ['query']),
        simple_composite_query: IDL.Func([], [StringQueryResult], ['query']),
        simple_query: IDL.Func([], [StringQueryResult], ['query']),
        simple_update: IDL.Func([], [StringQueryResult], []),
        update_query: IDL.Func([], [StringQueryResult], ['query'])
    });
};
export const init = ({ IDL }) => {
    return [];
};
