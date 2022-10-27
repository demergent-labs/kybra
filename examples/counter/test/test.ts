import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/counter/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/counter';

const counter_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code counter || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy counter`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(counter_canister)
];

run_tests(tests);
