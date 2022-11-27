export const idlFactory = ({ IDL }) => {
    return IDL.Service({
        get_notifier: IDL.Func(
            [],
            [IDL.Func([IDL.Vec(IDL.Nat8)], [], ['oneway'])],
            ['query']
        )
    });
};
export const init = ({ IDL }) => {
    return [];
};
