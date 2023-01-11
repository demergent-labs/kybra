import { Test } from 'azle/test';
import {
    Reaction,
    User,
    _SERVICE as CANISTER1_SERVICE
} from './dfx_generated/canister1/canister1.did';
import { _SERVICE as CANISTER2_SERVICE } from './dfx_generated/canister2/canister2.did';
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
    nat8,
    ok
} from 'azle';

type _SERVICE = CANISTER1_SERVICE | CANISTER2_SERVICE;

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

const STABLE_MAP_KEYS_COMPS: [
    (a: nat8 | undefined, b: nat8) => boolean,
    (a: nat16 | undefined, b: nat16) => boolean,
    (a: nat32 | undefined, b: nat32) => boolean,
    (a: Reaction | undefined, b: Reaction) => boolean,
    (a: User | undefined, b: User) => boolean,
    (a: string[] | undefined, b: string[]) => boolean,
    (a: BigUint64Array | undefined, b: BigInt64Array) => boolean,
    (a: null | undefined, b: null) => boolean,
    (a: boolean | undefined, b: boolean) => boolean,
    (a: float64 | undefined, b: float64) => boolean,
    (a: float32 | undefined, b: float32) => boolean,
    (a: nat | undefined, b: nat) => boolean,
    (a: blob | undefined, b: blob) => boolean,
    (a: string | undefined, b: string) => boolean
] = [
    simple_equals,
    simple_equals,
    simple_equals,
    (a, b) =>
        a !== undefined &&
        Object.keys(a).every((value) => Object.keys(b).includes(value)),
    (a, b) =>
        a !== undefined &&
        Object.keys(a).every((value) => Object.keys(b).includes(value)),
    (a, b) => a !== undefined && a.every((value, index) => value === b[index]),
    (a, b) => a !== undefined && a.every((value, index) => value === b[index]),
    simple_equals,
    simple_equals,
    simple_equals,
    (a, b) => a?.toFixed(2) === b.toFixed(2),
    simple_equals,
    (a, b) => a !== undefined && a.every((value, index) => value === b[index]),
    simple_equals
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

const STABLE_MAP_VALUES_COMPS: [
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
    (a, b) => (a === undefined ? true : a === b),
    (a, b) => (a === undefined ? true : a === b),
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
    stable_structures_canister: ActorSubclass<_SERVICE>,
    start: number,
    end: number
): Test[] {
    return [
        ...get_stable_map_empty_get_tests(
            '(on empty maps)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_is_empty_tests(
            true,
            '(when should be empty)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_len_tests(
            0n,
            '(when length should be 0)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_contains_key_tests(
            false,
            '(when there are no keys)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_keys_tests(
            0,
            '(when there are no keys)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_values_tests(
            0,
            '(when there are no values)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_items_tests(
            0,
            '(when there are no items)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_insert_tests(
            'initial insert',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_contains_key_tests(
            true,
            '(when it should contain the key)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_is_empty_tests(
            false,
            "(when it shouldn't be empty)",
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_len_tests(
            1n,
            '(when length should be 1)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_get_tests(
            '(when the values exist)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_keys_tests(
            1,
            '(when there are keys)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_values_tests(
            1,
            '(when there are values)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_items_tests(
            1,
            '(when there are items)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end)
    ];
}

export function get_second_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>,
    start: number,
    end: number
): Test[] {
    return [
        ...get_stable_map_get_tests(
            'post redeploy',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_keys_tests(
            1,
            '(when there are keys post redeploy)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_values_tests(
            1,
            '(when there are values post redeploy)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_items_tests(
            1,
            '(when there are items post redeploy)',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_remove_tests(
            'clean up',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end),
        ...get_stable_map_contains_key_tests(
            false,
            'post clean up',
            stable_structures_canister
        ).filter((_value, index) => index >= start && index <= end)
    ];
}

function get_stable_map_keys_tests(
    length: number,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((key, index) => {
        return {
            name: `stable_map_${index} keys ${suffix}`,
            test: async () => {
                let key_comp: (a: any, b: any) => boolean =
                    STABLE_MAP_KEYS_COMPS[index];
                const keys_result = await (stable_structures_canister as any)[
                    `stable_map_${index}_keys`
                ]();

                return {
                    ok:
                        (length === 0 && is_empty_array(keys_result)) ||
                        (length === 1 &&
                            key_comp(keys_result[0], STABLE_MAP_KEYS[index]))
                };
            }
        };
    });
}

function get_stable_map_values_tests(
    length: number,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_VALUES.map((key, index) => {
        return {
            name: `stable_map_${index} values ${suffix}`,
            test: async () => {
                let value_comp: (a: any, b: any) => boolean =
                    STABLE_MAP_VALUES_COMPS[index];
                const values_result = await (stable_structures_canister as any)[
                    `stable_map_${index}_values`
                ]();

                return {
                    ok:
                        (length === 0 && is_empty_array(values_result)) ||
                        (length === 1 &&
                            value_comp(
                                values_result[0],
                                STABLE_MAP_VALUES[index]
                            ))
                };
            }
        };
    });
}

function get_stable_map_items_tests(
    length: number,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((key, index) => {
        return {
            name: `stable_map_${index} items ${suffix}`,
            test: async () => {
                let key_comp: (a: any, b: any) => boolean =
                    STABLE_MAP_KEYS_COMPS[index];
                let value_comp: (a: any, b: any) => boolean =
                    STABLE_MAP_VALUES_COMPS[index];
                const items_result = await (stable_structures_canister as any)[
                    `stable_map_${index}_items`
                ]();

                return {
                    ok:
                        (length === 0 && is_empty_array(items_result)) ||
                        (length === 1 &&
                            key_comp(
                                items_result[0][0],
                                STABLE_MAP_KEYS[index]
                            ) &&
                            value_comp(
                                items_result[0][1],
                                STABLE_MAP_VALUES[index]
                            ))
                };
            }
        };
    });
}

function get_stable_map_insert_tests(
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((key, index) => {
        return {
            name: `stable_map_${index} insert value ${suffix}`,
            test: async () => {
                const insert_result = await (stable_structures_canister as any)[
                    `stable_map_${index}_insert`
                ](key, STABLE_MAP_VALUES[index]);

                return {
                    ok: ok(insert_result) && is_empty_opt(insert_result.ok)
                };
            }
        };
    });
}

function get_stable_map_contains_key_tests(
    should_contain: boolean,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((stable_map_key, index) => {
        return {
            name: `stable_map_${index} ${
                should_contain ? 'does' : 'does not'
            } contains key ${suffix}`,
            test: async () => {
                const set_result = await (stable_structures_canister as any)[
                    `stable_map_${index}_contains_key`
                ](stable_map_key);

                return {
                    ok: set_result === should_contain
                };
            }
        };
    });
}

function get_stable_map_empty_get_tests(
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((stable_map_key, index) => {
        return {
            name: `stable_map_${index} empty get ${suffix}`,
            test: async () => {
                const get_result = await (stable_structures_canister as any)[
                    `stable_map_${index}_get`
                ](stable_map_key);

                return {
                    ok: is_empty_opt(get_result)
                };
            }
        };
    });
}

function get_stable_map_get_tests(
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((stable_map_key, index) => {
        let value_comp: (a: any, b: any) => boolean =
            STABLE_MAP_VALUES_COMPS[index];
        return {
            name: `stable_map_${index} get test ${suffix}`,
            test: async () => {
                const get_result = await (stable_structures_canister as any)[
                    `stable_map_${index}_get`
                ](stable_map_key);

                return {
                    ok: value_comp(get_result[0], STABLE_MAP_VALUES[index])
                };
            }
        };
    });
}

function get_stable_map_is_empty_tests(
    should_be_empty: boolean,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((_value, index) => {
        return {
            name: `stable_map_${index} is empty test ${suffix}`,
            test: async () => {
                const result = await (stable_structures_canister as any)[
                    `stable_map_${index}_is_empty`
                ]();

                return {
                    ok: result === should_be_empty
                };
            }
        };
    });
}

function get_stable_map_len_tests(
    expected_len: nat64,
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((_value, index) => {
        return {
            name: `stable_map_${index} length test ${suffix}`,
            test: async () => {
                const result = await (stable_structures_canister as any)[
                    `stable_map_${index}_len`
                ]();

                return {
                    ok: result === expected_len
                };
            }
        };
    });
}

function get_stable_map_remove_tests(
    suffix: string,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((stable_map_keys, index) => {
        let value_comp: (a: any, b: any) => boolean =
            STABLE_MAP_VALUES_COMPS[index];
        return {
            name: `stable_map_${index} remove value test ${suffix}`,
            test: async () => {
                const get_result = await (stable_structures_canister as any)[
                    `stable_map_${index}_remove`
                ](stable_map_keys);

                return {
                    ok: value_comp(get_result[0], STABLE_MAP_VALUES[index])
                };
            }
        };
    });
}

/**
 * Determines whether the provided value is an empty array
 * @param value the value to test.
 * @returns `true` if the provided value is an empty array, `false` otherwise.
 */
function is_empty_array(value: any): boolean {
    return value?.length === 0;
}

/**
 * Determines whether the provided value is an Opt<T> or not.
 * @param value the value to test.
 * @returns `true` if the provided value is an empty Opt<T>, `false` otherwise.
 */
function is_empty_opt(value: any): boolean {
    return Array.isArray(value) && value.length === 0;
}
