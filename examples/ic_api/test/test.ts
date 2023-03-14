import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/ic_api/test/tests';
import { createActor } from './dfx_generated/ic_api';

const ic_api_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...get_tests(ic_api_canister as any).filter(
        (test) => test.name !== 'performance_counter'
    ),
    {
        name: 'performance_counter',
        test: async () => {
            const result = await ic_api_canister.performance_counter();

            return {
                ok: result >= 50_000n && result <= 90_000n
            };
        }
    }
];

run_tests(tests);
