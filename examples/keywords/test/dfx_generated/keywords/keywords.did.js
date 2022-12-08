export const idlFactory = ({ IDL }) => {
    const WithRecord = IDL.Record({
        with__: IDL.Bool,
        with___: IDL.Bool,
        with_______________________________________________________________________:
            IDL.Bool,
        with_: IDL.Bool,
        with______1: IDL.Bool
    });
    const SimpleRecord = IDL.Record({ from: IDL.Text });
    const KeywordVariant = IDL.Variant({
        is: IDL.Null,
        or: IDL.Null,
        and: IDL.Null,
        def: IDL.Null,
        del: IDL.Null,
        not: IDL.Null,
        import: IDL.Null,
        nonlocal: IDL.Null,
        finally: IDL.Null,
        True: IDL.Null,
        elif: IDL.Null,
        from: IDL.Null,
        class: IDL.Null,
        pass: IDL.Null,
        assert: IDL.Null,
        with: IDL.Null,
        lambda: IDL.Null,
        False: IDL.Null,
        global: IDL.Null,
        except: IDL.Null,
        raise: IDL.Null
    });
    const KeywordRecord = IDL.Record({
        as: IDL.Null,
        if: IDL.Bool,
        in: IDL.Bool,
        is: IDL.Float64,
        or: WithRecord,
        and: IDL.Int,
        def: IDL.Int32,
        del: IDL.Int64,
        for: IDL.Bool,
        not: SimpleRecord,
        try: IDL.Bool,
        import: IDL.Float32,
        return: IDL.Bool,
        nonlocal: IDL.Text,
        finally: IDL.Nat16,
        async: IDL.Reserved,
        await: IDL.Vec(IDL.Nat),
        continue: IDL.Vec(IDL.Int16),
        None: IDL.Tuple(IDL.Bool, IDL.Bool),
        True: IDL.Text,
        elif: IDL.Nat,
        else: IDL.Opt(IDL.Bool),
        from: IDL.Nat32,
        class: IDL.Int16,
        pass: KeywordVariant,
        assert: IDL.Int8,
        with: IDL.Principal,
        lambda: IDL.Opt(IDL.Text),
        False: IDL.Bool,
        global: IDL.Nat64,
        break: IDL.Opt(IDL.Vec(IDL.Nat8)),
        except: IDL.Nat8,
        while: IDL.Bool,
        raise: IDL.Vec(IDL.Nat8),
        yield: IDL.Bool
    });
    const RustKeywordRecord = IDL.Record({
        fn: IDL.Text,
        mut: IDL.Reserved,
        pub: IDL.Null,
        const: IDL.Int,
        crate: IDL.Nat,
        type: IDL.Vec(IDL.Nat8),
        become: IDL.Text,
        abstract: IDL.Bool
    });
    const RustKeywordVariant = IDL.Variant({
        fn: IDL.Null,
        pub: IDL.Null,
        type: IDL.Null
    });
    return IDL.Service({
        complex_keyword: IDL.Func([], [KeywordRecord], ['query']),
        keyword_variant: IDL.Func(
            [KeywordVariant],
            [KeywordVariant],
            ['query']
        ),
        rust_keyword: IDL.Func([], [RustKeywordRecord], ['query']),
        rust_keyword_variant: IDL.Func([], [RustKeywordVariant], ['query']),
        simple_keyword: IDL.Func([SimpleRecord], [SimpleRecord], ['query'])
    });
};
export const init = ({ IDL }) => {
    return [];
};
