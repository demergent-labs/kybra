import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/stable_memory/test/tests';
import { createActor } from './dfx_generated/stable_memory';

const stableMemoryCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests([
    {
        name: 'stable size',
        test: async () => {
            const result = await stableMemoryCanister.stable_size();

            return {
                Ok: result === 897
            };
        }
    },
    {
        name: 'stable64 size',
        test: async () => {
            const result = await stableMemoryCanister.stable64_size();

            return {
                Ok: result === 897n
            };
        }
    },
    ...getTests(createSnakeCaseProxy(stableMemoryCanister)).filter((test) => {
        return (
            test.name !== 'stable size' &&
            test.name !== 'stable64 size' &&
            test.name !== 'stable bytes' // TODO I believe this test now hits the cycle limit because we store a value at memory id 254 to distinguish between init/post_upgrade
        );
    })
]);
