import { Test } from 'azle/test';
import { _SERVICE } from './dfx_generated/stdlib/stdlib.did';
import { ActorSubclass } from '@dfinity/agent';

export function getTests(actor: ActorSubclass<_SERVICE>): Test[] {
    return [
        {
            name: 'test_base64',
            test: async () => {
                const result = await actor.test_base64();
                const base64String = Buffer.from(result).toString();
                const decodedString = Buffer.from(
                    base64String,
                    'base64'
                ).toString();

                return {
                    Ok: decodedString === 'Hello there sir'
                };
            }
        },
        {
            name: 'test_collections',
            test: async () => {
                const result = await actor.test_collections();
                return {
                    Ok: result === 'apple'
                };
            }
        },
        {
            name: 'test_datetime',
            test: async () => {
                const result = await actor.test_datetime();
                return {
                    Ok: !isNaN(Date.parse(result))
                };
            }
        },
        {
            name: 'test_itertools',
            test: async () => {
                const result = await actor.test_itertools();
                const expected = [
                    'ab',
                    'ac',
                    'ad',
                    'ba',
                    'bc',
                    'bd',
                    'ca',
                    'cb',
                    'cd',
                    'da',
                    'db',
                    'dc'
                ];
                return {
                    Ok: arrayEquals(result, expected)
                };
            }
        },
        {
            name: 'test_json',
            test: async () => {
                const result = await actor.test_json();
                return {
                    Ok: JSON.parse(result)['hello'] === 'world'
                };
            }
        },
        {
            name: 'test_random',
            test: async () => {
                const result = await actor.test_random();
                return {
                    Ok: result >= 0 && result <= 1
                };
            }
        },
        {
            name: 'test_string',
            test: async () => {
                const result = await actor.test_string();
                return {
                    Ok:
                        result ===
                        'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
                };
            }
        },
        {
            name: 'test_urllib',
            test: async () => {
                const result = await actor.test_urllib();
                return {
                    Ok:
                        result ===
                        'https://www.example.com/search?query=test&page=1'
                };
            }
        },
        {
            name: 'test_uuid',
            test: async () => {
                const result = await actor.test_uuid();
                return {
                    Ok: validateUUID(result)
                };
            }
        }
    ];
}

function arrayEquals(a: any[], b: any[]): boolean {
    return a.length === b.length && a.every((item, index) => item === b[index]);
}

function compareUint8Arrays(a: Uint8Array, b: Uint8Array): boolean {
    if (a.byteLength !== b.byteLength) return false;
    return a.every((val, i) => val === b[i]);
}

function validateUUID(uuid: string): boolean {
    // This regular expression checks if the string is a valid v4 UUID
    const pattern =
        /^([0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12})$/i;
    return pattern.test(uuid);
}
