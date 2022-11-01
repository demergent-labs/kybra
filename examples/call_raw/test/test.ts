import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/call_raw/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/call_raw';

const call_raw_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code call_raw || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy call_raw`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(call_raw_canister as any)
];

run_tests(tests);
