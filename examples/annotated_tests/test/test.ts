import { Test, getCanisterId, runTests } from 'azle/test';
import { createActor } from './dfx_generated/annotated_tests';
import { ActorSubclass } from '@dfinity/agent';
import { _SERVICE } from './dfx_generated/annotated_tests/annotated_tests.did';

const annotatedCanister = createActor(getCanisterId('annotated_tests'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(annotatedCanister));

function getTests(annotatedCanister: ActorSubclass<_SERVICE>): Test[] {
    return [
        {
            name: 'is_empty',
            test: async () => {
                return {
                    Ok: await annotatedCanister.is_empty()
                };
            }
        },
        {
            name: 'get_type_alias',
            test: async () => {
                return {
                    Ok: await annotatedCanister.get_type_alias()
                };
            }
        },
        {
            name: 'get_func',
            test: async () => {
                const result = await annotatedCanister.get_func();

                return {
                    Ok: result[1] === 'create_canister'
                };
            }
        }
    ];
}
