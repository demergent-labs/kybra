import { ActorSubclass } from '@dfinity/agent';
import { Test } from 'azle/test';
import { _SERVICE } from './dfx_generated/guard_functions/guard_functions.did';

export function get_tests(
    guard_functions_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        {
            name: 'accessible',
            test: async () => {
                const result = await guard_functions_canister.accessible();
                return {
                    ok: result === true
                };
            }
        },
        {
            name: 'guarded manual',
            test: async () => {
                const result = await guard_functions_canister.guarded_manual();
                return {
                    ok: result === true
                };
            }
        },
        {
            name: 'inaccessible',
            test: async () => {
                try {
                    const result =
                        await guard_functions_canister.inaccessible();
                    return {
                        ok: false
                    };
                } catch (err: any) {
                    return {
                        ok: err?.result?.reject_message == 'You shall not pass!'
                    };
                }
            }
        },
        {
            name: 'inaccessible update',
            test: async () => {
                try {
                    const result =
                        await guard_functions_canister.inaccessible_update();
                    return {
                        ok: false
                    };
                } catch (err: any) {
                    return {
                        ok: err.toString().includes('You shall not pass!')
                    };
                }
            }
        },
        {
            name: 'unguarded',
            test: async () => {
                const result = await guard_functions_canister.unguarded();
                return {
                    ok: result === true
                };
            }
        }
    ];
}
