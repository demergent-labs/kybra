export const idlFactory = ({ IDL }) => {
    const KeywordRecord = IDL.Record({
        as: IDL.Bool,
        if: IDL.Bool,
        in: IDL.Bool,
        is: IDL.Bool,
        or: IDL.Bool,
        and: IDL.Bool,
        def: IDL.Bool,
        del: IDL.Bool,
        for: IDL.Bool,
        not: IDL.Bool,
        try: IDL.Bool,
        import: IDL.Bool,
        return: IDL.Bool,
        nonlocal: IDL.Bool,
        finally: IDL.Bool,
        async: IDL.Bool,
        await: IDL.Bool,
        continue: IDL.Bool,
        None: IDL.Bool,
        True: IDL.Bool,
        elif: IDL.Bool,
        else: IDL.Bool,
        from: IDL.Bool,
        class: IDL.Bool,
        pass: IDL.Bool,
        assert: IDL.Bool,
        with: IDL.Bool,
        lambda: IDL.Bool,
        False: IDL.Bool,
        global: IDL.Bool,
        break: IDL.Bool,
        except: IDL.Bool,
        while: IDL.Bool,
        raise: IDL.Bool,
        yield: IDL.Bool
    });
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
    const WithRecord = IDL.Record({
        with__: IDL.Bool,
        with___: IDL.Bool,
        with_______________________________________________________________________:
            IDL.Bool,
        with_: IDL.Bool,
        with______1: IDL.Bool
    });
    const SimpleRecord = IDL.Record({ from: IDL.Text });
    return IDL.Service({
        complex_keyword: IDL.Func([], [KeywordRecord], ['query']),
        keyword_method: IDL.Func(
            [KeywordRecord, KeywordVariant, WithRecord],
            [IDL.Tuple(KeywordRecord, KeywordVariant, WithRecord)],
            ['query']
        ),
        simple_keyword: IDL.Func([SimpleRecord], [SimpleRecord], ['query'])
    });
};
export const init = ({ IDL }) => {
    return [];
};
