export const idlFactory = ({ IDL }) => {
  const BlogPost = IDL.Record({ 'title' : IDL.Text });
  const User = IDL.Record({
    'username' : IDL.Text,
    'blog_posts' : IDL.Vec(BlogPost),
  });
  const Reaction = IDL.Variant({ 'Sad' : IDL.Null, 'Happy' : IDL.Null });
  return IDL.Service({
    'get_stable_map_0' : IDL.Func([IDL.Nat8], [IDL.Opt(IDL.Text)], ['query']),
    'get_stable_map_1' : IDL.Func(
        [IDL.Nat16],
        [IDL.Opt(IDL.Vec(IDL.Nat8))],
        ['query'],
      ),
    'get_stable_map_10' : IDL.Func(
        [IDL.Float32],
        [IDL.Opt(IDL.Opt(IDL.Bool))],
        ['query'],
      ),
    'get_stable_map_11' : IDL.Func([IDL.Nat], [IDL.Opt(User)], ['query']),
    'get_stable_map_12' : IDL.Func(
        [IDL.Vec(IDL.Nat8)],
        [IDL.Opt(Reaction)],
        ['query'],
      ),
    'get_stable_map_13' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(IDL.Principal)],
        ['query'],
      ),
    'get_stable_map_2' : IDL.Func([IDL.Nat32], [IDL.Opt(IDL.Nat)], ['query']),
    'get_stable_map_3' : IDL.Func([Reaction], [IDL.Opt(IDL.Int)], ['query']),
    'get_stable_map_4' : IDL.Func([User], [IDL.Opt(IDL.Float32)], ['query']),
    'get_stable_map_5' : IDL.Func(
        [IDL.Opt(IDL.Text)],
        [IDL.Opt(IDL.Float64)],
        ['query'],
      ),
    'get_stable_map_6' : IDL.Func(
        [IDL.Vec(IDL.Nat64)],
        [IDL.Opt(IDL.Bool)],
        ['query'],
      ),
    'get_stable_map_7' : IDL.Func([IDL.Null], [IDL.Opt(IDL.Null)], ['query']),
    'get_stable_map_8' : IDL.Func([IDL.Bool], [IDL.Opt(IDL.Null)], ['query']),
    'get_stable_map_9' : IDL.Func(
        [IDL.Float64],
        [IDL.Opt(IDL.Vec(IDL.Text))],
        ['query'],
      ),
    'set_stable_map_0' : IDL.Func([IDL.Nat8, IDL.Text], [], []),
    'set_stable_map_1' : IDL.Func([IDL.Nat16, IDL.Vec(IDL.Nat8)], [], []),
    'set_stable_map_10' : IDL.Func([IDL.Float32, IDL.Opt(IDL.Bool)], [], []),
    'set_stable_map_11' : IDL.Func([IDL.Nat, User], [], []),
    'set_stable_map_12' : IDL.Func([IDL.Vec(IDL.Nat8), Reaction], [], []),
    'set_stable_map_13' : IDL.Func([IDL.Text, IDL.Principal], [], []),
    'set_stable_map_2' : IDL.Func([IDL.Nat32, IDL.Nat], [], []),
    'set_stable_map_3' : IDL.Func([Reaction, IDL.Int], [], []),
    'set_stable_map_4' : IDL.Func([User, IDL.Float32], [], []),
    'set_stable_map_5' : IDL.Func([IDL.Opt(IDL.Text), IDL.Float64], [], []),
    'set_stable_map_6' : IDL.Func([IDL.Vec(IDL.Nat64), IDL.Bool], [], []),
    'set_stable_map_7' : IDL.Func([IDL.Null, IDL.Null], [], []),
    'set_stable_map_8' : IDL.Func([IDL.Bool, IDL.Null], [], []),
    'set_stable_map_9' : IDL.Func([IDL.Float64, IDL.Vec(IDL.Text)], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
