import { getCanisterId, runTests, Test } from 'azle/test';
import { createActor } from './dfx_generated/generators';

const generatorsCanister = createActor(getCanisterId('generators'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests());

function getTests(): Test[] {
    return [
        {
            name: 'get_randomness_directly',
            test: async () => {
                const result =
                    await generatorsCanister.get_randomness_directly();

                return {
                    Ok: result.length === 32
                };
            }
        },
        {
            name: 'get_randomness_indirectly',
            test: async () => {
                const result =
                    await generatorsCanister.get_randomness_indirectly();

                return {
                    Ok: result.length === 32
                };
            }
        },
        {
            name: 'get_randomness_super_indirectly',
            test: async () => {
                const result =
                    await generatorsCanister.get_randomness_super_indirectly();

                return {
                    Ok: result.length === 96
                };
            }
        }
    ];
}
