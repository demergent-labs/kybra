export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'caller' : IDL.Func([], [IDL.Principal], ['query']),
    'performance_counter' : IDL.Func([], [IDL.Nat64], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
