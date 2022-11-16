export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'caller' : IDL.Func([], [IDL.Principal], ['query']) });
};
export const init = ({ IDL }) => { return []; };
