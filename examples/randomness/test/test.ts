import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/randomness/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/randomness';

const randomnessCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(
    getTests(createSnakeCaseProxy(randomnessCanister)).map((test) => {
        if (test.name === 'dfx deploy') {
            return {
                name: 'dfx deploy',
                prep: async () => {
                    execSync('dfx deploy');
                    await new Promise((resolve) => setTimeout(resolve, 10_000));
                }
            };
        }

        return test;
    })
);
