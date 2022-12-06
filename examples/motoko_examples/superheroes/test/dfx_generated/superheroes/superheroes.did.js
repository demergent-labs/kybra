export const idlFactory = ({ IDL }) => {
    const Box = IDL.Rec();
    Box.fill(IDL.Opt(IDL.Tuple(IDL.Text, Box)));
    const Superhero = IDL.Record({
        superpowers: IDL.Opt(IDL.Tuple(IDL.Text, Box)),
        name: IDL.Text
    });
    return IDL.Service({
        create: IDL.Func([Superhero], [IDL.Nat32], []),
        delete_hero: IDL.Func([IDL.Nat32], [IDL.Bool], []),
        read: IDL.Func([IDL.Nat32], [IDL.Opt(Superhero)], ['query']),
        update_: IDL.Func([IDL.Nat32, Superhero], [IDL.Bool], [])
    });
};
export const init = ({ IDL }) => {
    return [];
};
