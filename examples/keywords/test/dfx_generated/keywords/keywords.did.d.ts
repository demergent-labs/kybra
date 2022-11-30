import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface KeywordRecord {
    as: boolean;
    if: boolean;
    in: boolean;
    is: boolean;
    or: boolean;
    and: boolean;
    def: boolean;
    del: boolean;
    for: boolean;
    not: boolean;
    try: boolean;
    import: boolean;
    return: boolean;
    nonlocal: boolean;
    finally: boolean;
    async: boolean;
    await: boolean;
    continue: boolean;
    None: boolean;
    True: boolean;
    elif: boolean;
    else: boolean;
    from: boolean;
    class: boolean;
    pass: boolean;
    assert: boolean;
    with: boolean;
    lambda: boolean;
    False: boolean;
    global: boolean;
    break: boolean;
    except: boolean;
    while: boolean;
    raise: boolean;
    yield: boolean;
}
export type KeywordVariant =
    | { is: null }
    | { or: null }
    | { and: null }
    | { def: null }
    | { del: null }
    | { not: null }
    | { import: null }
    | { nonlocal: null }
    | { finally: null }
    | { True: null }
    | { elif: null }
    | { from: null }
    | { class: null }
    | { pass: null }
    | { assert: null }
    | { with: null }
    | { lambda: null }
    | { False: null }
    | { global: null }
    | { except: null }
    | { raise: null };
export interface SimpleRecord {
    from: string;
}
export interface WithRecord {
    with__: boolean;
    with___: boolean;
    with_______________________________________________________________________: boolean;
    with_: boolean;
    with______1: boolean;
}
export interface _SERVICE {
    complex_keyword: ActorMethod<[], KeywordRecord>;
    keyword_method: ActorMethod<
        [KeywordRecord, KeywordVariant, WithRecord],
        [KeywordRecord, KeywordVariant, WithRecord]
    >;
    simple_keyword: ActorMethod<[SimpleRecord], SimpleRecord>;
}
