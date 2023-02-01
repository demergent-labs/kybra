import { ActorSubclass } from '@dfinity/agent';
import { Test } from 'azle/test';
import { _SERVICE } from './dfx_generated/function_guards/function_guards.did';

export function get_tests(
    function_guard_canister: ActorSubclass<_SERVICE>
): Test[] {
    return [
        {
            name: 'accessible',
            test: async () => {
                const result = await function_guard_canister.accessible();
                return {
                    ok: result === true
                };
            }
        },
        {
            name: 'guarded manual',
            test: async () => {
                const result = await function_guard_canister.guarded_manual();
                return {
                    ok: result === true
                };
            }
        },
        {
            name: 'inaccessible',
            test: async () => {
                try {
                    const result = await function_guard_canister.inaccessible();
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
            name: 'unguarded',
            test: async () => {
                const result = await function_guard_canister.unguarded();
                return {
                    ok: result === true
                };
            }
        }
    ];
}
