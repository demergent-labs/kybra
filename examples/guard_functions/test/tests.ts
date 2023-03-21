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
                    Ok: result === true
                };
            }
        },
        {
            name: 'guarded manual',
            test: async () => {
                const result = await guard_functions_canister.guarded_manual();
                return {
                    Ok: result === true
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
                        Ok: false
                    };
                } catch (err: any) {
                    return {
                        Ok: err?.result?.reject_message == 'You shall not pass!'
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
                        Ok: false
                    };
                } catch (err: any) {
                    return {
                        Ok: err.toString().includes('You shall not pass!')
                    };
                }
            }
        },
        {
            name: 'unguarded',
            test: async () => {
                const result = await guard_functions_canister.unguarded();
                return {
                    Ok: result === true
                };
            }
        }
    ];
}
