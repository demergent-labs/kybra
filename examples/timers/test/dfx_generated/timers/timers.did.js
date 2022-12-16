export const idlFactory = ({ IDL }) => {
  const TimerIds = IDL.Record({
    'repeat' : IDL.Nat64,
    'inline1' : IDL.Nat64,
    'inline2' : IDL.Nat64,
    'single' : IDL.Nat64,
    'inner' : IDL.Nat64,
  });
  const StatusReport = IDL.Record({
    'repeat' : IDL.Nat8,
    'inline1' : IDL.Nat8,
    'inline2' : IDL.Nat8,
    'single' : IDL.Bool,
    'inner' : IDL.Text,
  });
  return IDL.Service({
    'clear_timer' : IDL.Func([IDL.Nat64], [], []),
    'set_timers' : IDL.Func([IDL.Nat64, IDL.Nat64], [TimerIds], []),
    'status_report' : IDL.Func([], [StatusReport], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
