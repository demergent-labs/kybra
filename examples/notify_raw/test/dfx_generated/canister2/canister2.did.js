export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_notified' : IDL.Func([], [IDL.Bool], ['query']),
    'receive_notification' : IDL.Func([], [IDL.Bool], []),
  });
};
export const init = ({ IDL }) => { return []; };
