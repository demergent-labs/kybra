export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_strftime' : IDL.Func([], [IDL.Text], ['query']),
    'get_time' : IDL.Func([], [IDL.Float64], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
