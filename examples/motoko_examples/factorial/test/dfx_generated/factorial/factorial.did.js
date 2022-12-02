export const idlFactory = ({ IDL }) => {
    return IDL.Service({ fac: IDL.Func([IDL.Nat], [IDL.Nat], []) });
};
export const init = ({ IDL }) => {
    return [];
};
