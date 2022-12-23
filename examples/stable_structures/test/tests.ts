import { Test } from 'azle/test';
import {
    BlogPost,
    Reaction,
    User,
    _SERVICE
} from './dfx_generated/stable_structures/stable_structures.did';
import { ActorSubclass } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import {
    blob,
    float32,
    float64,
    int,
    nat,
    nat16,
    nat32,
    nat64,
    nat8
} from 'azle';

const HELLO_BYTES = [104, 101, 108, 108, 111];
const STABLE_MAP_KEYS: [
    nat8,
    nat16,
    nat32,
    Reaction,
    User,
    string[], //Opt?
    BigUint64Array,
    null,
    boolean,
    float64,
    float32,
    nat,
    blob,
    string
] = [
    0,
    0,
    0,
    {
        Happy: null
    },
    {
        username: 'username',
        blog_posts: [
            {
                title: 'MyBlogPost'
            }
        ]
    },
    ['world'],
    new BigUint64Array([1n, 2n, 3n, 4n, 5n]),
    null,
    false,
    0,
    10.23,
    0n,
    new Uint8Array(HELLO_BYTES),
    'hello'
];
const STABLE_MAP_VALUES: [
    string,
    blob,
    nat,
    int,
    float32,
    float64,
    boolean,
    null,
    null,
    string[],
    boolean[],
    User,
    Reaction,
    Principal
] = [
    'hello',
    new Uint8Array(HELLO_BYTES),
    2n,
    2n,
    4,
    5,
    true,
    null,
    null,
    ['hello', 'world'],
    [true],
    {
        username: 'username2',
        blog_posts: [
            {
                title: 'BlagPost'
            }
        ]
    },
    { Sad: null },
    Principal.fromText('aaaaa-aa')
];

function simple_equals(a: any, b: any): boolean {
    return a === b;
}

const STABLE_MAP_VALUE_COMPS: [
    (a: string | undefined, b: string) => boolean,
    (a: blob | undefined, b: blob) => boolean,
    (a: nat | undefined, b: nat) => boolean,
    (a: int | undefined, b: int) => boolean,
    (a: float32 | undefined, b: float32) => boolean,
    (a: float64 | undefined, b: float64) => boolean,
    (a: boolean | undefined, b: boolean) => boolean,
    (a: null | undefined, b: null) => boolean,
    (a: null | undefined, b: null) => boolean,
    (a: string[] | undefined, b: string[]) => boolean,
    (a: boolean[] | undefined, b: boolean[]) => boolean,
    (a: User | undefined, b: User) => boolean,
    (a: Reaction | undefined, b: Reaction) => boolean,
    (a: Principal | undefined, b: Principal) => boolean
] = [
    simple_equals,
    (a, b) => a !== undefined && a.every((value, index) => value === b[index]),
    simple_equals,
    simple_equals,
    simple_equals,
    simple_equals,
    simple_equals,
    simple_equals,
    simple_equals,
    (a, b) => a !== undefined && a.every((value, index) => value === b[index]),
    (a, b) => a !== undefined && a.every((value, index) => value === b[index]),
    (a, b) =>
        a !== undefined &&
        a.username === b.username &&
        a.blog_posts[0].title === b.blog_posts[0].title,
    (a, b) =>
        a !== undefined &&
        Object.keys(a).every((value) => Object.keys(b).includes(value)),
    (a, b) => a !== undefined && a.toText() === b.toText()
];

export function get_first_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        ...get_empty_read_tests(
            'initial read',
            stable_structures_canister
        ).filter(
            (value) =>
                !value.name.includes('stable_map_7') &&
                !value.name.includes('stable_map_8')
        ),
        ...get_is_empty_tests(
            true,
            'initial read',
            stable_structures_canister
        ).filter(
            (value) =>
                !value.name.includes('stable_map_7') &&
                !value.name.includes('stable_map_8')
        ),
        ...get_len_tests(0n, 'initial read', stable_structures_canister).filter(
            (value) =>
                !value.name.includes('stable_map_7') &&
                !value.name.includes('stable_map_8')
        ),
        ...get_contains_key_tests(
            false,
            'initial read',
            stable_structures_canister
        ).filter(
            (value) =>
                !value.name.includes('stable_map_7') &&
                !value.name.includes('stable_map_8')
        ),
        ...get_set_value_tests('initial set', stable_structures_canister),
        ...get_contains_key_tests(true, 'post set', stable_structures_canister),
        ...get_is_empty_tests(false, 'post set', stable_structures_canister),
        ...get_len_tests(1n, 'post set', stable_structures_canister),
        ...get_get_tests('post set', stable_structures_canister).filter(
            (value) =>
                !value.name.includes('stable_map_7') &&
                !value.name.includes('stable_map_8')
        )
    ];
}

export function get_second_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        ...get_get_tests('post redeploy', stable_structures_canister).filter(
            (value) =>
                !value.name.includes('stable_map_7') &&
                !value.name.includes('stable_map_8')
        ),
        ...get_remove_tests('clean up', stable_structures_canister).filter(
            (value) =>
                !value.name.includes('stable_map_7') &&
                !value.name.includes('stable_map_8')
        ),
        ...get_contains_key_tests(
            false,
            'post clean up',
            stable_structures_canister
        ).filter(
            (value) =>
                !value.name.includes('stable_map_7') &&
                !value.name.includes('stable_map_8')
        )
    ];
}

function get_set_value_tests(
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((key, index) =>
        generate_set_value_tests(
            index,
            key,
            STABLE_MAP_VALUES[index],
            suffix,
            stable_structures_canister
        )
    );
}

function get_contains_key_tests(
    should_contain: boolean,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((value, index) =>
        generate_contains_key_test(
            index,
            value,
            should_contain,
            suffix,
            stable_structures_canister
        )
    );
}

function get_empty_read_tests(
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((value, index) =>
        generate_empty_read_tests(
            index,
            value,
            suffix,
            stable_structures_canister
        )
    );
}

function get_get_tests(
    suffix: string,
    stable_structures: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((value, index) =>
        generate_get_value_test(
            index,
            value,
            STABLE_MAP_VALUES[index],
            STABLE_MAP_VALUE_COMPS[index],
            suffix,
            stable_structures
        )
    );
}

function get_is_empty_tests(
    should_be_empty: boolean,
    suffix: string,
    stable_structures: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((_value, index) =>
        generate_is_empty_test(
            index,
            should_be_empty,
            suffix,
            stable_structures
        )
    );
}

function get_remove_tests(
    suffix: string,
    stable_structures: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((value, index) =>
        generate_remove_test(
            index,
            value,
            STABLE_MAP_VALUES[index],
            STABLE_MAP_VALUE_COMPS[index],
            suffix,
            stable_structures
        )
    );
}

function get_len_tests(
    expected_len: nat64,
    suffix: string,
    stable_structures: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((_value, index) =>
        generate_len_test(index, expected_len, suffix, stable_structures)
    );
}

function generate_get_value_test(
    num: number,
    key: any,
    value: any,
    value_comp: (a: any, b: any) => boolean,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} get test for sure automated ${suffix}`,
        test: async () => {
            const get_result = await (stable_structures_canister as any)[
                `get_stable_map_${num}`
            ](key);

            return {
                ok: value_comp(get_result[0], value)
            };
        }
    };
}

function generate_contains_key_test(
    num: number,
    key: any,
    should_contain: boolean,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} contains key that exists ${suffix}`,
        test: async () => {
            const set_result = await (stable_structures_canister as any)[
                `contains_key_stable_map_${num}`
            ](key);

            return {
                ok: set_result === should_contain
            };
        }
    };
}

function generate_set_value_tests(
    num: number,
    key: any,
    value: any,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} set value ${suffix}`,
        test: async () => {
            const set_result = await (stable_structures_canister as any)[
                `set_stable_map_${num}`
            ](key, value);

            return {
                ok: set_result === undefined
            };
        }
    };
}

function generate_empty_read_tests(
    num: number,
    key: any,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} initial read ${suffix}`,
        test: async () => {
            const get_result = await (stable_structures_canister as any)[
                `get_stable_map_${num}`
            ](key);

            return {
                ok: get_result[0] === undefined
            };
        }
    };
}

function generate_len_test(
    num: number,
    length: nat64,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} length test ${suffix}`,
        test: async () => {
            const result = await (stable_structures_canister as any)[
                `len_stable_map_${num}`
            ]();

            console.log('the resutl is');
            console.log(result);
            console.log('the resutl shoul be');
            console.log(length);

            return {
                ok: result === length
            };
        }
    };
}

function generate_is_empty_test(
    num: number,
    should_be_empty: boolean,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} is empty test ${suffix}`,
        test: async () => {
            const result = await (stable_structures_canister as any)[
                `is_empty_stable_map_${num}`
            ]();

            console.log('the resutl is');
            console.log(result);
            console.log('the resutl shoul be');
            console.log(should_be_empty);

            return {
                ok: result === should_be_empty
            };
        }
    };
}

function generate_remove_test(
    num: number,
    key: any,
    value: any,
    value_comp: (a: any, b: any) => boolean,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} remove value test ${suffix}`,
        test: async () => {
            const get_result = await (stable_structures_canister as any)[
                `remove_stable_map_${num}`
            ](key);

            return {
                ok: value_comp(get_result[0], value)
            };
        }
    };
}
