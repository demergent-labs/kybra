import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface KeywordRecord {
    as: null;
    if: boolean;
    in: boolean;
    is: number;
    or: WithRecord;
    and: bigint;
    def: number;
    del: bigint;
    for: boolean;
    not: SimpleRecord;
    try: boolean;
    import: number;
    return: boolean;
    nonlocal: string;
    finally: number;
    async: any;
    await: Array<bigint>;
    continue: Int16Array;
    None: [boolean, boolean];
    True: string;
    elif: bigint;
    else: [] | [boolean];
    from: number;
    class: number;
    pass: KeywordVariant;
    assert: number;
    with: Principal;
    lambda: [] | [string];
    False: boolean;
    global: bigint;
    break: [] | [Uint8Array];
    except: number;
    while: boolean;
    raise: Uint8Array;
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
    keyword_variant: ActorMethod<[KeywordVariant], KeywordVariant>;
    simple_keyword: ActorMethod<[SimpleRecord], SimpleRecord>;
}
