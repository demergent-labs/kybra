import { Test } from 'azle/test';
import {
    BlogPost,
    Reaction,
    User,
    _SERVICE
} from './dfx_generated/stable_structures/stable_structures.did';
import { ActorSubclass } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

const HELLO_BYTES = [104, 101, 108, 108, 111];
const STABLE_MAP_KEYS = [
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
const WRONG_STABLE_MAP_KEYS = [
    100,
    100,
    100,
    {
        Sad: null
    },
    {
        username: 'wrong_username',
        blog_posts: [
            {
                title: 'MyBlogPost'
            }
        ]
    },
    ['wrong'],
    new BigUint64Array([100n, 200n, 300n, 400n, 500n]),
    null,
    true,
    100,
    100,
    100n,
    new Uint8Array([100, 200, 300, 400, 500]),
    'wrong'
];

export function get_set_value_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((value, index) =>
        generate_set_value_tests(index, value, stable_structures_canister)
    );
}

export function get_contains_key_that_exists_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((value, index) =>
        generate_contains_key_that_exist_test(
            index,
            value,
            stable_structures_canister
        )
    );
}

export function get_contains_key_that_doesnt_exists_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return WRONG_STABLE_MAP_KEYS.map((value, index) =>
        generate_contains_key_that_doesnt_exist_test(
            index,
            value,
            stable_structures_canister
        )
    );
}

export function get_contains_key_that_doesnt_exists_yet_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return WRONG_STABLE_MAP_KEYS.map((value, index) =>
        generate_contains_key_that_doesnt_exist_test(
            index,
            value,
            stable_structures_canister
        )
    );
}

export function get_pre_value_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return STABLE_MAP_KEYS.map((value, index) =>
        generate_pre_value_tests(index, value, stable_structures_canister)
    );
}

export function get_get_tests(
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        {
            name: 'stable_map_0 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_0(0);

                return {
                    ok: get_result[0] === 'hello'
                };
            }
        },
        {
            name: 'stable_map_1 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_1(0);

                return {
                    ok: new Uint8Array(HELLO_BYTES).every(
                        (value, index) => value === get_result[0]?.[index]
                    )
                };
            }
        },
        {
            name: 'stable_map_2 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_2(0);

                return {
                    ok: get_result[0] === 2n
                };
            }
        },
        {
            name: 'stable_map_3 get test',
            test: async () => {
                let key: Reaction = {
                    Happy: null
                };
                const get_result =
                    await stable_structures_canister.get_stable_map_3(key);

                return {
                    ok: get_result[0] === 2n
                };
            }
        },
        {
            name: 'stable_map_4 get test',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'MyBlogPost'
                };
                let key: User = {
                    username: 'username',
                    blog_posts: [blog_post]
                };
                const get_result =
                    await stable_structures_canister.get_stable_map_4(key);

                return {
                    ok: get_result[0] === 4
                };
            }
        },
        {
            name: 'stable_map_5 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_5([
                        'world'
                    ]);

                return {
                    ok: get_result[0] === 5
                };
            }
        },
        {
            name: 'stable_map_6 get test',
            test: async () => {
                let key = new BigUint64Array([1n, 2n, 3n, 4n, 5n]);

                const get_result =
                    await stable_structures_canister.get_stable_map_6(key);

                return {
                    ok: get_result[0] === true
                };
            }
        },
        {
            name: 'stable_map_7 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_7(null);

                return {
                    ok: get_result[0] === null
                };
            }
        },
        {
            name: 'stable_map_8 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_8(false);

                return {
                    ok: get_result[0] === null
                };
            }
        },
        {
            name: 'stable_map_9 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_9(0);

                return {
                    ok:
                        get_result[0]?.length == 2 &&
                        get_result[0][0] === 'hello' &&
                        get_result[0][1] === 'world'
                };
            }
        },
        {
            name: 'stable_map_10 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_10(10.23);

                return {
                    ok: get_result[0]?.[0] === true
                };
            }
        },
        {
            name: 'stable_map_11 get test',
            test: async () => {
                let blog_post: BlogPost = {
                    title: 'BlagPost'
                };
                let user: User = {
                    username: 'username2',
                    blog_posts: [blog_post]
                };

                const get_result =
                    await stable_structures_canister.get_stable_map_11(0n);

                return {
                    ok:
                        get_result[0]?.username === user.username &&
                        get_result[0]?.blog_posts?.[0]?.title ===
                            blog_post.title
                };
            }
        },
        {
            name: 'stable_map_12 get test',
            test: async () => {
                let value: Reaction = {
                    Sad: null
                };

                const get_result =
                    await stable_structures_canister.get_stable_map_12(
                        new Uint8Array(HELLO_BYTES)
                    );

                return {
                    ok: get_result[0] != undefined && 'Sad' in get_result[0]
                };
            }
        },
        {
            name: 'stable_map_13 get test',
            test: async () => {
                const get_result =
                    await stable_structures_canister.get_stable_map_13('hello');

                return {
                    ok:
                        get_result[0]?.toText() ===
                        Principal.fromText('aaaaa-aa').toText()
                };
            }
        }
    ];
}

function generate_contains_key_that_exist_test(
    num: number,
    key: any,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} contains key that exits`,
        test: async () => {
            const set_result = await (stable_structures_canister as any)[
                `contains_key_stable_map_${num}`
            ](key);

            return {
                ok: set_result
            };
        }
    };
}

function generate_contains_key_that_doesnt_exist_test(
    num: number,
    key: any,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} contains key that doesnt exits`,
        test: async () => {
            const set_result = await (stable_structures_canister as any)[
                `contains_key_stable_map_${num}`
            ](key);

            return {
                ok: !set_result
            };
        }
    };
}

function generate_set_value_tests(
    num: number,
    key: any,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} set value`,
        test: async () => {
            const set_result = (stable_structures_canister as any)[
                `set_stable_map_${num}`
            ](key);

            return {
                ok: set_result[0] === undefined
            };
        }
    };
}

function generate_pre_value_tests(
    num: number,
    key: any,
    stable_structures_canister: ActorSubclass<_SERVICE>
): Test {
    return {
        name: `stable_map_${num} initial read auto generated`,
        test: async () => {
            const get_result = (stable_structures_canister as any)[
                `get_stable_map_${num}`
            ](key);

            return {
                ok: get_result[0] === undefined
            };
        }
    };
}
