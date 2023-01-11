export const idlFactory = ({ IDL }) => {
    const KeyTooLarge = IDL.Variant({ max: IDL.Nat32, given: IDL.Nat32 });
    const InsertError = IDL.Variant({
        ValueTooLarge: KeyTooLarge,
        KeyTooLarge: KeyTooLarge
    });
    const StableMap10InsertResult = IDL.Variant({
        ok: IDL.Opt(IDL.Opt(IDL.Bool)),
        err: InsertError
    });
    const BlogPost = IDL.Record({ title: IDL.Text });
    const User = IDL.Record({
        username: IDL.Text,
        blog_posts: IDL.Vec(BlogPost)
    });
    const StableMap11InsertResult = IDL.Variant({
        ok: IDL.Opt(User),
        err: InsertError
    });
    const Reaction = IDL.Variant({ Sad: IDL.Null, Happy: IDL.Null });
    const StableMap12InsertResult = IDL.Variant({
        ok: IDL.Opt(Reaction),
        err: InsertError
    });
    const StableMap13InsertResult = IDL.Variant({
        ok: IDL.Opt(IDL.Principal),
        err: InsertError
    });
    const StableMap7InsertResult = IDL.Variant({
        ok: IDL.Opt(IDL.Null),
        err: InsertError
    });
    const StableMap9InsertResult = IDL.Variant({
        ok: IDL.Opt(IDL.Vec(IDL.Text)),
        err: InsertError
    });
    return IDL.Service({
        stable_map_10_contains_key: IDL.Func(
            [IDL.Float32],
            [IDL.Bool],
            ['query']
        ),
        stable_map_10_get: IDL.Func(
            [IDL.Float32],
            [IDL.Opt(IDL.Opt(IDL.Bool))],
            ['query']
        ),
        stable_map_10_insert: IDL.Func(
            [IDL.Float32, IDL.Opt(IDL.Bool)],
            [StableMap10InsertResult],
            []
        ),
        stable_map_10_is_empty: IDL.Func([], [IDL.Bool], ['query']),
        stable_map_10_items: IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(IDL.Float32, IDL.Opt(IDL.Bool)))],
            ['query']
        ),
        stable_map_10_keys: IDL.Func([], [IDL.Vec(IDL.Float32)], ['query']),
        stable_map_10_len: IDL.Func([], [IDL.Nat64], ['query']),
        stable_map_10_remove: IDL.Func(
            [IDL.Float32],
            [IDL.Opt(IDL.Opt(IDL.Bool))],
            []
        ),
        stable_map_10_values: IDL.Func(
            [],
            [IDL.Vec(IDL.Opt(IDL.Bool))],
            ['query']
        ),
        stable_map_11_contains_key: IDL.Func([IDL.Nat], [IDL.Bool], ['query']),
        stable_map_11_get: IDL.Func([IDL.Nat], [IDL.Opt(User)], ['query']),
        stable_map_11_insert: IDL.Func(
            [IDL.Nat, User],
            [StableMap11InsertResult],
            []
        ),
        stable_map_11_is_empty: IDL.Func([], [IDL.Bool], ['query']),
        stable_map_11_items: IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(IDL.Nat, User))],
            ['query']
        ),
        stable_map_11_keys: IDL.Func([], [IDL.Vec(IDL.Nat)], ['query']),
        stable_map_11_len: IDL.Func([], [IDL.Nat64], ['query']),
        stable_map_11_remove: IDL.Func([IDL.Nat], [IDL.Opt(User)], []),
        stable_map_11_values: IDL.Func([], [IDL.Vec(User)], ['query']),
        stable_map_12_contains_key: IDL.Func(
            [IDL.Vec(IDL.Nat8)],
            [IDL.Bool],
            ['query']
        ),
        stable_map_12_get: IDL.Func(
            [IDL.Vec(IDL.Nat8)],
            [IDL.Opt(Reaction)],
            ['query']
        ),
        stable_map_12_insert: IDL.Func(
            [IDL.Vec(IDL.Nat8), Reaction],
            [StableMap12InsertResult],
            []
        ),
        stable_map_12_is_empty: IDL.Func([], [IDL.Bool], ['query']),
        stable_map_12_items: IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(IDL.Vec(IDL.Nat8), Reaction))],
            ['query']
        ),
        stable_map_12_keys: IDL.Func(
            [],
            [IDL.Vec(IDL.Vec(IDL.Nat8))],
            ['query']
        ),
        stable_map_12_len: IDL.Func([], [IDL.Nat64], ['query']),
        stable_map_12_remove: IDL.Func(
            [IDL.Vec(IDL.Nat8)],
            [IDL.Opt(Reaction)],
            []
        ),
        stable_map_12_values: IDL.Func([], [IDL.Vec(Reaction)], ['query']),
        stable_map_13_contains_key: IDL.Func([IDL.Text], [IDL.Bool], ['query']),
        stable_map_13_get: IDL.Func(
            [IDL.Text],
            [IDL.Opt(IDL.Principal)],
            ['query']
        ),
        stable_map_13_insert: IDL.Func(
            [IDL.Text, IDL.Principal],
            [StableMap13InsertResult],
            []
        ),
        stable_map_13_is_empty: IDL.Func([], [IDL.Bool], ['query']),
        stable_map_13_items: IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Principal))],
            ['query']
        ),
        stable_map_13_keys: IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
        stable_map_13_len: IDL.Func([], [IDL.Nat64], ['query']),
        stable_map_13_remove: IDL.Func(
            [IDL.Text],
            [IDL.Opt(IDL.Principal)],
            []
        ),
        stable_map_13_values: IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
        stable_map_7_contains_key: IDL.Func([IDL.Null], [IDL.Bool], ['query']),
        stable_map_7_get: IDL.Func([IDL.Null], [IDL.Opt(IDL.Null)], ['query']),
        stable_map_7_insert: IDL.Func(
            [IDL.Null, IDL.Null],
            [StableMap7InsertResult],
            []
        ),
        stable_map_7_is_empty: IDL.Func([], [IDL.Bool], ['query']),
        stable_map_7_items: IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(IDL.Null, IDL.Null))],
            ['query']
        ),
        stable_map_7_keys: IDL.Func([], [IDL.Vec(IDL.Null)], ['query']),
        stable_map_7_len: IDL.Func([], [IDL.Nat64], ['query']),
        stable_map_7_remove: IDL.Func([IDL.Null], [IDL.Opt(IDL.Null)], []),
        stable_map_7_values: IDL.Func([], [IDL.Vec(IDL.Null)], ['query']),
        stable_map_8_contains_key: IDL.Func([IDL.Bool], [IDL.Bool], ['query']),
        stable_map_8_get: IDL.Func([IDL.Bool], [IDL.Opt(IDL.Null)], ['query']),
        stable_map_8_insert: IDL.Func(
            [IDL.Bool, IDL.Null],
            [StableMap7InsertResult],
            []
        ),
        stable_map_8_is_empty: IDL.Func([], [IDL.Bool], ['query']),
        stable_map_8_items: IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(IDL.Bool, IDL.Null))],
            ['query']
        ),
        stable_map_8_keys: IDL.Func([], [IDL.Vec(IDL.Bool)], ['query']),
        stable_map_8_len: IDL.Func([], [IDL.Nat64], ['query']),
        stable_map_8_remove: IDL.Func([IDL.Bool], [IDL.Opt(IDL.Null)], []),
        stable_map_8_values: IDL.Func([], [IDL.Vec(IDL.Null)], ['query']),
        stable_map_9_contains_key: IDL.Func(
            [IDL.Float64],
            [IDL.Bool],
            ['query']
        ),
        stable_map_9_get: IDL.Func(
            [IDL.Float64],
            [IDL.Opt(IDL.Vec(IDL.Text))],
            ['query']
        ),
        stable_map_9_insert: IDL.Func(
            [IDL.Float64, IDL.Vec(IDL.Text)],
            [StableMap9InsertResult],
            []
        ),
        stable_map_9_is_empty: IDL.Func([], [IDL.Bool], ['query']),
        stable_map_9_items: IDL.Func(
            [],
            [IDL.Vec(IDL.Tuple(IDL.Float64, IDL.Vec(IDL.Text)))],
            ['query']
        ),
        stable_map_9_keys: IDL.Func([], [IDL.Vec(IDL.Float64)], ['query']),
        stable_map_9_len: IDL.Func([], [IDL.Nat64], ['query']),
        stable_map_9_remove: IDL.Func(
            [IDL.Float64],
            [IDL.Opt(IDL.Vec(IDL.Text))],
            []
        ),
        stable_map_9_values: IDL.Func(
            [],
            [IDL.Vec(IDL.Vec(IDL.Text))],
            ['query']
        )
    });
};
export const init = ({ IDL }) => {
    return [];
};