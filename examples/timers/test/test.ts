import { run_tests, Test } from 'azle/test';
import { get_tests } from './tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/timers';

const timers_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code timers || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy timers`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(timers_canister as any).filter(
        (test) => test.name !== 'two_tuple_with_inline_records' // Kybra does not have the concept of inline records
    )
];

run_tests(tests);
