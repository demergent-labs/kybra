export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_current_message' : IDL.Func([], [IDL.Text], ['query']),
    'simple_update' : IDL.Func([IDL.Text], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
