import { run_tests, Test } from 'azle/test';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/stable_structures';
import {
    get_get_tests,
    get_pre_value_tests,
    get_set_value_tests
} from './tests';

const stable_structures_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code stable_structures || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy stable_structures`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_pre_value_tests(stable_structures_canister),
    ...get_set_value_tests(stable_structures_canister),
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx deploy stable_structures`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_get_tests(stable_structures_canister)
];

run_tests(tests);
