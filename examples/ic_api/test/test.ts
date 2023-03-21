import { runTests, Test } from 'azle/test';
import { getTests } from 'azle/examples/ic_api/test/tests';
import { createActor } from './dfx_generated/ic_api';

const icApiCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...getTests(icApiCanister as any).filter(
        (test) => test.name !== 'performance_counter'
    ),
    {
        name: 'performance_counter',
        test: async () => {
            const result = await icApiCanister.performance_counter();

            return {
                Ok: result >= 50_000n && result <= 90_000n
            };
        }
    }
];

runTests(tests);
